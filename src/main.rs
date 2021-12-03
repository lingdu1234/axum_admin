use poem::endpoint::Files;
use poem::middleware::AddData;
use poem::{listener::TcpListener, middleware::Cors, EndpointExt, Result, Route, Server};
use std::time::Duration;

use sea_orm_casbin_adapter::{casbin::prelude::*, SeaOrmAdapter};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt, subscribe::CollectExt, EnvFilter};

//导入全局
pub use crate::config::CFG;
//路由日志追踪
use crate::middleware::{Auth, PoemTracer};
use crate::utils::casbin_service::CasbinService;

mod apps;
//  配置文件
mod config;
// 数据库
mod db;
mod env;
mod middleware;
pub mod utils;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", &CFG.log.log_level);
    }
    //日志追踪 将log转换到Tracing统一输出
    LogTracer::init().unwrap();
    // 系统变量设置
    env::setup();

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
    //  数据库联机
    let db = db::db_connect().await;
    //  casbin设置
    let casbin_service = CasbinService::new(db.clone())
        .await
        .expect("casbin init error");
    // let e = &casbin_service.enforcer;
    // let e_result = e.enforce(("a", "b", "c")).unwrap();
    // println!("e_result-----------{}", e_result);
    // -------------------------------------------------------

    //  跨域
    let cors = Cors::new();
    //  Swagger
    let listener = TcpListener::bind(&CFG.server.address);
    // 启动app  注意中间件顺序 最后的先执行，尤其AddData 顺序不对可能会导致数据丢失，无法在某些位置获取数据

    let app = Route::new()
        .nest(
            "/api",
            apps::api()
                .with(PoemTracer)
                .with(Auth)
                .with(AddData::new(db))
                .with(AddData::new(casbin_service.clone()))
                .with(cors),
        )
        .nest(
            "/",
            Files::new(&CFG.web.dir).index_file(&CFG.web.index),
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
