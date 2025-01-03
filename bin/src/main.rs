use std::{net::SocketAddr, path::PathBuf, time::Duration};

//
use app_service::{service_utils, tasks};
use axum::{
    handler::HandlerWithoutStateExt,
    http::{Method, StatusCode},
    routing::get_service,
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use configs::CFG;
use tokio::signal;
use tower_http::{
    compression::{predicate::NotForContentType, CompressionLayer, DefaultPredicate, Predicate},
    cors::{Any, CorsLayer},
    services::ServeDir,
};
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};
use utils::my_env::{self, RT};
// 路由日志追踪

// #[tokio::main]
fn main() {
    RT.block_on(async {
        if std::env::var_os("RUST_LOG").is_none() {
            std::env::set_var("RUST_LOG", &CFG.log.log_level);
        }
        my_env::setup();

        // 系统变量设置
        let log_env = my_env::get_log_level();

        //  日志设置
        let format = my_env::get_log_format();

        // 文件输出
        let file_appender = tracing_appender::rolling::hourly(&CFG.log.dir, &CFG.log.file);
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

        // 标准控制台输出
        let (std_non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
        let logger = Registry::default()
            .with(EnvFilter::from_default_env().add_directive(log_env.into()))
            .with(fmt::Layer::default().with_writer(std_non_blocking).event_format(format.clone()).pretty())
            .with(fmt::Layer::default().with_writer(non_blocking).event_format(format))
            // .with(console_layer)
            ;
        tracing::subscriber::set_global_default(logger).unwrap();

        // apis全局初始化
        service_utils::ApiUtils::init_all_api().await;
        // 定时任务初始化
        tasks::timer_task_init().await.expect("定时任务初始化失败");

        //  跨域
        let cors = CorsLayer::new()
            .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
            .allow_origin(Any)
            .allow_headers(Any);
        // 顺序不对可能会导致数据丢失，无法在某些位置获取数据
        let static_files_service = get_service(
            ServeDir::new(&CFG.web.dir)
                .not_found_service(handle_404.into_service())
                .append_index_html_on_directories(true),
        );
        let handle = axum_server::Handle::new();

        let shutdown_future = shutdown_signal(handle.clone());
        tokio::spawn(shutdown_future);
        let app = Router::new().nest_service(&CFG.server.api_prefix, api::api()).fallback_service(static_files_service);

        let app = match &CFG.server.content_gzip {
            true => {
                //  开启压缩后 SSE 数据无法返回  text/event-stream 单独处理不压缩
                let predicate = DefaultPredicate::new().and(NotForContentType::new("text/event-stream"));
                app.layer(CompressionLayer::new().compress_when(predicate))
            }
            false => app,
        };
        let app = app.layer(cors);
        let addr: SocketAddr = CFG.server.address.clone().parse().unwrap();
        match CFG.server.ssl {
            true => {
                let config = RustlsConfig::from_pem_file(PathBuf::from(&CFG.cert.cert), PathBuf::from(&CFG.cert.key)).await.unwrap();

                tracing::debug!("listening on {addr}");
                axum_server::bind_rustls(addr, config).handle(handle).serve(app.into_make_service()).await.unwrap()
            }

            false => axum_server::bind(addr).serve(app.into_make_service()).await.unwrap(),
        }
    })
}

async fn handle_404() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not found")
}

async fn shutdown_signal(handle: axum_server::Handle) {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Received termination signal shutting down");
    handle.graceful_shutdown(Some(Duration::from_secs(10))); // 10 secs is how
                                                             // long docker will
                                                             // wait
                                                             // to force shutdown
}
