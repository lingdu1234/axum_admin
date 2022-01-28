use poem::endpoint::StaticFilesEndpoint;
use poem::{listener::TcpListener, middleware::Cors, EndpointExt, Result, Route, Server};
use std::time::Duration;
use tracing_subscriber::fmt::time::LocalTime;

use tracing_log::LogTracer;
use tracing_subscriber::{fmt, subscribe::CollectExt, EnvFilter};

//导入全局
pub use crate::config::CFG;
use crate::database::{db_conn, DB};

//路由日志追踪
use crate::middleware::Tracing;
use crate::utils::get_enforcer;

mod apps;
//  配置文件
mod config;
// 数据库
mod database;
mod env;
mod middleware;
pub mod utils;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", &CFG.log.log_level);
    }
    env::setup();

    //日志追踪 将log转换到Tracing统一输出
    LogTracer::init().unwrap();

    // 系统变量设置

    //  日志设置
    let format = fmt::format()
        .with_level(true) // don't include levels in formatted output
        .with_target(true) // don't include targets
        .with_thread_ids(true) // include the thread ID of the current thread
        .with_thread_names(true) // include the name of the current thread
        .with_timer(LocalTime::rfc_3339()) // use RFC 3339 timestamps
        .compact();
    let file_appender = tracing_appender::rolling::daily(&CFG.log.dir, &CFG.log.file); //文件输出设置
                                                                                       //文件输出
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    //标准控制台输出
    let (std_non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
    let collector = tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with(
            fmt::Subscriber::new()
                .event_format(format.clone())
                .with_writer(std_non_blocking)
                .pretty(),
        )
        .with(
            fmt::Subscriber::new()
                .event_format(format)
                .with_writer(non_blocking)
                .pretty(),
        );
    tracing::collect::set_global_default(collector).expect("Unable to set a global collector");
    //  数据库联机

    // 数据库初始化
    database::db_init().await;
    //  casbin设置
    get_enforcer().await;
    //     .await
    //     .expect("casbin init error");
    // let e = &casbin_service.enforcer;
    // let e_result = e.enforce(("a", "b", "c")).unwrap();
    // println!("e_result-----------{}", e_result);
    // -------------------------------------------------------

    //  跨域
    let cors = Cors::new();
    println!("dvsdvdsvdsv------------------------------");
    //  Swagger
    let listener = TcpListener::bind(&CFG.server.address);
    // 启动app  注意中间件顺序 最后的先执行，尤其AddData 顺序不对可能会导致数据丢失，无法在某些位置获取数据

    let app = Route::new()
        .nest("/api", apps::api())
        .nest(
            "/",
            StaticFilesEndpoint::new(&CFG.web.dir).index_file(&CFG.web.index),
        )
        .with(Tracing)
        .with(cors);

    let server = Server::new(listener).name("poem-admin");
    tracing::info!("Server started");
    server
        .run_with_graceful_shutdown(
            app,
            async move {
                let _ = tokio::signal::ctrl_c().await;
            },
            Some(Duration::from_secs(1)),
        )
        .await
}
