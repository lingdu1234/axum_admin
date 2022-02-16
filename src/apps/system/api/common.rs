use poem::{handler, web::Json, Result};

use crate::apps::{
    common::models::{CaptchaImage, Res},
    system::service,
};

#[handler]
pub fn get_captcha() -> Result<Json<Res<CaptchaImage>>> {
    let res = service::common::get_captcha();
    Ok(Json(Res::with_data(res)))
}
