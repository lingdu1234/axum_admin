use configs::CFG;
use tracing::Level;
use tracing_subscriber::{
    fmt,
    fmt::format::{Compact, Format},
};

pub fn setup() {
    //   打印logo
    self::show_log();
}

fn show_log() {
    let logo = r#"
                                           | |         (_)
     _ __   ___   ___ _ __ ___     __ _  __| |_ __ ___  _ _ __
    | '_ \ / _ \ / _ \ '_ ` _ \   / _` |/ _` | '_ ` _ \| | '_ \
    | |_) | (_) |  __/ | | | | | | (_| | (_| | | | | | | | | | |
    | .__/ \___/ \___|_| |_| |_|  \__,_|\__,_|_| |_| |_|_|_| |_|
    | |
    |_|
       "#;
    println!("{}", logo);
    // println!("系统架构：{}", std::env::var("OS").unwrap().to_string());
    // println!("系统类型：{}", std::env::consts::ARCH);
    // println!("操作系统：{}", std::env::consts::FAMILY);
    // println!()
}

pub fn get_log_level() -> Level {
    match CFG.log.log_level.as_str() {
        "TRACE" => tracing::Level::TRACE,
        "DEBUG" => tracing::Level::DEBUG,
        "INFO" => tracing::Level::INFO,
        "WARN" => tracing::Level::WARN,
        "ERROR" => tracing::Level::ERROR,
        _ => tracing::Level::INFO,
    }
}

#[cfg(target_os = "windows")]
use time::format_description::well_known::Rfc3339;
#[cfg(target_os = "windows")]
use tracing_subscriber::fmt::time::LocalTime;
#[cfg(target_os = "windows")]
pub fn get_log_format() -> Format<Compact, LocalTime<Rfc3339>> {
    fmt::format()
        .with_level(true) // don't include levels in formatted output
        .with_target(true) // don't include targets
        .with_thread_ids(true) // include the thread ID of the current thread
        // .with_thread_names(true)
        // .with_file(true)
        // .with_ansi(true)
        // .with_line_number(true) // include the name of the current thread
        .with_timer(LocalTime::rfc_3339()) // use RFC 3339 timestamps
        .compact()
}

#[cfg(not(target_os = "windows"))]
pub fn get_log_format() -> Format<Compact> {
    fmt::format().with_level(true).with_target(true).with_thread_ids(true).compact()
}
