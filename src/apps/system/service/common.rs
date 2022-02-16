use captcha_rust::Captcha;

use crate::{apps::common::models::CaptchaImage, utils};

pub fn get_captcha() -> CaptchaImage {
    let captcha = Captcha::new(5, 130, 40);
    let uuid = utils::encrypt_password(&captcha.text, "");
    CaptchaImage {
        captcha_on_off: true,
        uuid,
        img: captcha.base_img,
    }
}
