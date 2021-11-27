use std::time::Duration;

use poem::middleware::AddData;
use poem::{listener::TcpListener, middleware::Cors, EndpointExt, Result, Route, Server};
use poem::{IntoResponse, Response};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt, subscribe::CollectExt, EnvFilter};

// casbin
// use sqlx_adapter::casbin::prelude::*;
// // use sqlx_adapter::casbin::Result;
// use sqlx_adapter::SqlxAdapter;

//导入全局
pub use crate::config::CFG;
//路由日志追踪
use crate::middleware::PoemTracer;

mod apps;
//  配置文件
mod config;
// 数据库
mod db;
mod env;
mod middleware;
mod resp;
mod tests;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "debug");
    }
    //日志追踪 将log转换到Tracing统一输出
    LogTracer::init().unwrap();
    // 系统变量设置
    env::setup();
    //  数据库联机
    let db = db::db_connect().await;
    // let m = DefaultModel::from_file("config/rbac_model.conf")
    //     .await
    //     .unwrap();
    // // mysql://root:lingdu515639@127.0.0.1:13306/wk_data
    // // postgres://postgres:lingdu515639@127.0.0.1:25432/wk
    // let adpt = SqlxAdapter::new("mysql://root:lingdu515639@127.0.0.1:13306/wk_data", 8)
    //     .await
    //     .unwrap();
    // let mut e = Enforcer::new(m, adpt).await.unwrap();
    //  日志设置
    let file_appender = tracing_appender::rolling::daily(&CFG.log.dir, &CFG.log.file); //文件输出设置
                                                                                       //文件输出
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    //标准控制台输出
    let (std_non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
    let collector = tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::TRACE.into()))
        .with(
            fmt::Subscriber::new()
                .with_writer(std_non_blocking)
                .pretty(),
        )
        .with(fmt::Subscriber::new().with_writer(non_blocking).pretty());
    tracing::collect::set_global_default(collector).expect("Unable to set a global collector");
    //  跨域
    let cors = Cors::new();
    //  Swagger
    let listener = TcpListener::bind("127.0.0.1:3000");
    // 启动app
    let app = Route::new()
        .nest(
            "/",
            apps::api()
                .with(PoemTracer)
                .with(AddData::new(db))
                .with(cors),
        )
        // .after(|mut resp| async move {
        //     if resp.status() != StatusCode::OK {
        //         resp.set_status(StatusCode::OK);
        //         // let b = resp.take_body();
        //         resp
        //     } else {
        //         resp
        //     }
        // })
        ;

    let server = Server::new(listener).name("poem-admin");
    tracing::info!("Server started");
    server
        .run_with_graceful_shutdown(
            app,
            async move {
                let _ = tokio::signal::ctrl_c().await;
            },
            Some(Duration::from_secs(5)),
        )
        .await
}
