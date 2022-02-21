use captcha_rust::Captcha;
use db::common::captcha::CaptchaImage;

use crate::utils;

pub fn get_captcha() -> CaptchaImage {
    let captcha = Captcha::new(4, 130, 40);
    let uuid = utils::encrypt_password(&captcha.text, "");
    CaptchaImage {
        captcha_on_off: true,
        uuid,
        img: captcha.base_img,
    }
}
