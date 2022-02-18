use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CaptchaImage {
    pub captcha_on_off: bool,
    pub uuid: String,
    pub img: String,
}
