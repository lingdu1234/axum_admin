use poem::{listener::TcpListener, middleware::Cors, EndpointExt, Route, Server};
use tracing_subscriber::{fmt, subscribe::CollectExt, EnvFilter};

mod apps;
mod config;
mod middleware;

//将CFG导入全局
use crate::config::CFG;
use crate::middleware::PoemTracer;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem-admin=debug");
    }

    //  日志
    // let file_appender = tracing_appender::rolling::daily("data/log/", "log");
    let file_appender = tracing_appender::rolling::daily(&CFG.log.dir, &CFG.log.file);
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let (std_non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());

    //
    // tracing_subscriber::fmt::init();
    //

    //
    //  只有一个生效
    // let a = tracing_subscriber::fmt()
    //     .with_writer(std::io::stdout)
    //     .with_writer(non_blocking)
    //     .init();
    //
    let collector = tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::TRACE.into()))
        .with(fmt::Subscriber::new().with_writer(std_non_blocking))
        .with(fmt::Subscriber::new().with_writer(non_blocking));
    tracing::collect::set_global_default(collector).expect("Unable to set a global collector");
    //  跨域
    let cors = Cors::new();

    let app = Route::new().nest("/", apps::api().with(PoemTracer).with(cors));
    let listener = TcpListener::bind("127.0.0.1:3000");
    let server = Server::new(listener).await?;
    tracing::warn!("Server started");
    tracing::trace!("Server started~~~~~~~~~~~~~~~~~~~~");
    server.run(app).await
}
