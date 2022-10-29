use std::{convert::Infallible, time::Duration};

use axum::response::sse::{Event, Sse};
use db::{
    common::{captcha::CaptchaImage, res::Res},
    system::models::server_info::SysInfo,
};
use futures::stream::{self, Stream};
use tokio_stream::StreamExt as _;

use super::super::service::server_info::get_oper_sys_info;

pub async fn get_captcha() -> Res<CaptchaImage> {
    let res = super::super::service::common::get_captcha();
    Res::with_data(res)
}

pub async fn get_server_info() -> Res<SysInfo> {
    let res = get_oper_sys_info();

    Res::with_data(res)
}


//  这个不知道为啥有问题
pub async fn get_server_info_sse() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::repeat_with(|| {
        let sys_info = get_oper_sys_info();
        Event::default().data(serde_json::to_string(&sys_info).unwrap_or_else(|_| "0".to_string()))
    }).map(Ok)
    .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::new().interval(Duration::from_secs(5)))
}
