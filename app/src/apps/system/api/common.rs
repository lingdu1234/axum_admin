use std::{convert::Infallible, time::Duration};

use axum::response::sse::{Event, Sse};
use db::{
    common::{captcha::CaptchaImage, res::Res},
    system::models::server_info::SysInfo,
};
use futures::stream::{self, Stream};
use tokio_stream::StreamExt as _;

use super::super::service::server_info::get_oper_sys_info;


#[utoipa::path(
    get,
    path = "/comm/get_captcha",
    tag = "common",
    responses(
        (status = 200, description = "获取验证码", body = CaptchaImage)
    )
)]
/// 验证码获取
pub async fn get_captcha() -> Res<CaptchaImage> {
    let res = super::super::service::common::get_captcha();
    Res::with_data(res)
}

#[utoipa::path(
    get,
    path = "/system/monitor/server",
    tag = "SysMonitor",
    responses(
        (status = 200, description = "服务器信息", body = SysInfo)
    ),
    security(("authorization" = []))
)]
/// 获取服务器信息
pub async fn get_server_info() -> Res<SysInfo> {
    let res = get_oper_sys_info();

    Res::with_data(res)
}

#[utoipa::path(
    get,
    path = "/system/monitor/server-event",
    tag = "SysMonitor",
    responses(
        (status = 200, description = "服务器信息", body = String)
    ),
    security(("authorization" = []))
)]
/// 获取服务器信息 SSE
pub async fn get_server_info_sse() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::repeat_with(|| {
        let sys_info = get_oper_sys_info();
        Event::default().data(serde_json::to_string(&sys_info).unwrap_or_else(|_| "0".to_string()))
    })
    .map(Ok)
    .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::new().interval(Duration::from_secs(1)).text("keep-alive-text"))
}
