use captcha_rust::Captcha;

use crate::utils;

use crate::apps::common::models::CaptchaImage;

pub fn get_captcha() -> CaptchaImage {
    let captcha = Captcha::new(5, 130, 40);
    let uuid = utils::encrypt_password(&captcha.text, "");
    let captcha_image = CaptchaImage {
        captcha_on_off: true,
        uuid,
        img: captcha.base_img,
    };
    captcha_image
}
