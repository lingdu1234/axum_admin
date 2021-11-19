// use tracing::Collect;
// use tracing_appender::non_blocking::NonBlocking;
// use tracing_subscriber::fmt::format::{DefaultFields, Format};
// use tracing_subscriber::fmt::Subscriber;
// use tracing_subscriber::subscribe::Layered;
// use tracing_subscriber::{fmt, subscribe::CollectExt, EnvFilter, Registry};
//
// use crate::CFG;
//
// pub fn log_init() -> Layered<
//     Subscriber<
//         Layered<
//             Subscriber<Layered<EnvFilter, Registry>, DefaultFields, Format, NonBlocking>,
//             Layered<EnvFilter, Registry>,
//         >,
//         DefaultFields,
//         Format,
//         NonBlocking,
//     >,
//     Layered<
//         Subscriber<Layered<EnvFilter, Registry>, DefaultFields, Format, NonBlocking>,
//         Layered<EnvFilter, Registry>,
//     >,
// > {
//     //  日志
//     let file_appender = tracing_appender::rolling::daily(&CFG.log.dir, &CFG.log.file);
//     let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
//     let (std_non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
//
//     //
//     // tracing_subscriber::fmt::init();
//     //
//
//     //
//     //  只有一个生效
//     // let a = tracing_subscriber::fmt()
//     //     .with_writer(std::io::stdout)
//     //     .with_writer(non_blocking)
//     //     .init();
//     //
//     let collector = tracing_subscriber::registry()
//         .with(EnvFilter::from_default_env().add_directive(tracing::Level::TRACE.into()))
//         .with(fmt::Subscriber::new().with_writer(std_non_blocking))
//         .with(fmt::Subscriber::new().with_writer(non_blocking));
//     collector
//     // tracing::collect::set_global_default(collector).expect("Unable to set a global collector");
// }
