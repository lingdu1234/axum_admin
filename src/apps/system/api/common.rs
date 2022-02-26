use db::common::{captcha::CaptchaImage, res::Res};
use poem::handler;

#[handler]
pub fn get_captcha() -> Res<CaptchaImage> {
    let res = super::super::service::common::get_captcha();
    Res::with_data(res)
}
