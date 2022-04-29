use std::time::Duration;

use app::{
    apps,
    my_env::{self, RT},
    tasks,
    utils::{self, cert::CERT_KEY},
};
use configs::CFG;
//
use poem::{
    endpoint::StaticFilesEndpoint,
    listener::{Listener, RustlsConfig, TcpListener},
    middleware::{
        Compression,
        Cors, // ,TokioMetrics
    },
    EndpointExt, Result, Route, Server,
};
use poem::listener::RustlsCertificate;
use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter, Registry};

// 路由日志追踪

// #[tokio::main]
fn main() -> Result<(), std::io::Error> {
    RT.block_on(async {
        if std::env::var_os("RUST_LOG").is_none() {
            std::env::set_var("RUST_LOG", &CFG.log.log_level);
        }
        my_env::setup();
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
        //  跨域
        let cors = Cors::new();
        // let metrics = TokioMetrics::new();
        // 启动app  注意中间件顺序 最后的先执行，尤其AddData
        // 顺序不对可能会导致数据丢失，无法在某些位置获取数据

        let app = Route::new()
            .nest(&CFG.server.api_prefix, apps::api())
            .nest("/", StaticFilesEndpoint::new(&CFG.web.dir).show_files_listing().index_file(&CFG.web.index))
            // .at("/mtc", metrics.exporter())
            // .with(Tracing)
            .with_if(CFG.server.content_gzip, Compression::new())
            // .with(metrics)
            .with(cors);

        match CFG.server.ssl {
            true => {
                let listener = TcpListener::bind(&CFG.server.address).rustls(RustlsConfig::new().fallback(RustlsCertificate::new().key(&*CERT_KEY.key).cert(&*CERT_KEY.cert)));
                let server = Server::new(listener).name(&CFG.server.name);
                server
                    // .run(app)
                    .run_with_graceful_shutdown(
                        app,
                        async move {
                            let _ = tokio::signal::ctrl_c().await;
                        },
                        Some(Duration::from_secs(1)),
                    )
                    .await
            }
            false => {
                let listener = TcpListener::bind(&CFG.server.address);
                let server = Server::new(listener).name(&CFG.server.name);
                server
                    // .run(app)
                    .run_with_graceful_shutdown(
                        app,
                        async move {
                            let _ = tokio::signal::ctrl_c().await;
                        },
                        Some(Duration::from_secs(1)),
                    )
                    .await
            }
        }
    })
}
