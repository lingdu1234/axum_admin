use poem::middleware::AddData;
use poem::{listener::TcpListener, middleware::Cors, EndpointExt, Result, Route, Server};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt, subscribe::CollectExt, EnvFilter};

//将CFG导入全局
use crate::config::CFG;
//路由日志追踪
use crate::middleware::PoemTracer;

mod apps;
mod config;
mod database;
mod env;
mod middleware;
mod tests;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // 系统变量设置
    env::setup();
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "debug");
    }
    LogTracer::init().unwrap(); //日志追踪 将log转换到Tracing统一输出

    let db = database::db_connect().await; //  数据库联机
                                           //  日志设置
    let file_appender = tracing_appender::rolling::daily(&CFG.log.dir, &CFG.log.file); //文件输出设置
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender); //文件输出
    let (std_non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout()); //标准控制台输出
    let collector = tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::TRACE.into()))
        .with(fmt::Subscriber::new().with_writer(std_non_blocking))
        .with(fmt::Subscriber::new().with_writer(non_blocking));
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
            // .with(AddData::new(100))
            // .with(AddData::new(200_u32))
            .with(AddData::new(db))
            .with(cors)
            // .after(|res| async move {
            //     println!("hahaha{:?}", res);
            // }),
    );

    let server = Server::new(listener).await?;
    tracing::info!("Server started");
    server.run(app).await
}
