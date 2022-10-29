use std::time::Duration;

use db::{
    common::{captcha::CaptchaImage, res::Res},
    system::models::server_info::SysInfo,
};
use futures::StreamExt;
use poem::{
    handler,
    web::sse::{Event, SSE},
};

use super::super::service::server_info::get_oper_sys_info;

#[handler]
pub async fn get_captcha() -> Res<CaptchaImage> {
    let res = super::super::service::common::get_captcha();
    Res::with_data(res)
}

#[handler]
pub async fn get_server_info() -> Res<SysInfo> {
    let res = get_oper_sys_info();
    Res::with_data(res)
}

#[handler]
pub async fn get_server_info_ws() -> SSE {
    SSE::new(tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(1))).map(move |_| {
        let sys_info = get_oper_sys_info();
        Event::message(serde_json::to_string(&sys_info).unwrap_or_else(|_| "".to_string()))
    }))
    .keep_alive(Duration::from_secs(5))
}
