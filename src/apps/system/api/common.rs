use db::common::{captcha::CaptchaImage, res::Res};
use poem::{handler, web::Json};

#[handler]
pub fn get_captcha() -> Json<Res<CaptchaImage>> {
    let res = super::super::service::common::get_captcha();
    Json(Res::with_data(res))
}
