use db::{
    common::{captcha::CaptchaImage, res::Res},
    system::models::server_info::SysInfo,
};
use poem::handler;

use super::super::service::server_info::{get_oper_sys_info, SYSINFO};

#[handler]
pub async fn get_captcha() -> Res<CaptchaImage> {
    let res = super::super::service::common::get_captcha();
    Res::with_data(res)
}

#[handler]
pub async fn get_server_info() -> Res<SysInfo> {
    let sys_info = SYSINFO.lock().await;
    let info = match &*sys_info {
        Some(sys_info) => sys_info.clone(),
        None => {
            let res = get_oper_sys_info().await;
            res
        }
    };
    Res::with_data(info)
}
