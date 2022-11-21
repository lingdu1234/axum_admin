// use std::time::Duration;

use std::{net::SocketAddr, str::FromStr};

//
use app::{
    api_doc::OpenApiDoc,
    apps,
    my_env::{self, RT},
    tasks, utils,
};
use axum::{
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
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};
// 路由日志追踪

// #[tokio::main]
fn main() {
    RT.block_on(async {
        if std::env::var_os("RUST_LOG").is_none() {
            std::env::set_var("RUST_LOG", &CFG.log.log_level);
        }
        my_env::setup();
        // let console_layer = console_subscriber::ConsoleLayer::builder()
        //     .retention(Duration::from_secs(60))
        //     .server_addr(([127, 0, 0, 1], 5555))
        //     .spawn();
        // let console_layer = console_subscriber::spawn();

        //  设置日志追踪
        // if &CFG.log.log_level == "TRACE" {
        //     LogTracer::builder()
        //         .with_max_level(log::LevelFilter::Trace)
        //         .init()
        //         .unwrap();
        // }

        // 系统变量设置
        let log_env = my_env::get_log_level();

        //  日志设置
        let format = my_env::get_log_format();

        // 文件输出
        let file_appender = tracing_appender::rolling::hourly(&CFG.log.dir, &CFG.log.file);
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
        // let json_layer = fmt::layer().event_format(fmt::format().json()).fmt_fields(JsonFields::new()).with_writer(non_blocking).pretty();

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
        utils::ApiUtils::init_all_api().await;
        // 定时任务初始化
        tasks::timer_task_init().await.expect("定时任务初始化失败");

        let addr = SocketAddr::from_str(&CFG.server.address).unwrap();
        //  跨域
        let cors = CorsLayer::new()
            .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
            .allow_origin(Any)
            .allow_headers(Any);
        // 顺序不对可能会导致数据丢失，无法在某些位置获取数据
        let static_files_service = get_service(ServeDir::new(&CFG.web.dir).append_index_html_on_directories(true))
            .handle_error(|error: std::io::Error| async move { (StatusCode::INTERNAL_SERVER_ERROR, format!("Unhandled internal error: {}", error)) });

        let app = Router::new()
            //  "/" 与所有路由冲突
            .fallback(static_files_service)
            .nest(&CFG.server.api_prefix, apps::api())
            .merge(SwaggerUi::new("/ui/*tail").url(Url::new("api", "/api-doc/openapi.json"), OpenApiDoc::openapi()));
        println!("{}", OpenApiDoc::openapi().to_pretty_json().unwrap());
        let app = match &CFG.server.content_gzip {
            true => {
                //  开启压缩后 SSE 数据无法返回  text/event-stream 单独处理不压缩
                let predicate = DefaultPredicate::new().and(NotForContentType::new("text/event-stream"));
                app.layer(CompressionLayer::new().compress_when(predicate))
            }
            false => app,
        };
        let app = app.layer(cors);

        match CFG.server.ssl {
            true => {
                let config = RustlsConfig::from_pem_file(&CFG.cert.cert, &CFG.cert.key).await.unwrap();
                axum_server::bind_rustls(addr, config).serve(app.into_make_service()).await.unwrap()
            }

            false => axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .with_graceful_shutdown(shutdown_signal())
                .await
                .unwrap(),
        }
    })
}

async fn shutdown_signal() {
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

    println!("signal received, starting graceful shutdown");
}
