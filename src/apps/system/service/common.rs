use anyhow::Result;
use captcha_rust::Captcha;
use configs::CFG;
use db::common::captcha::CaptchaImage;
use poem::web::Multipart;
use tokio::{fs, io::AsyncWriteExt};

use crate::utils;

/// 获取验证码
pub fn get_captcha() -> CaptchaImage {
    let captcha = Captcha::new(4, 130, 40);
    let uuid = utils::encrypt_password(&captcha.text, "");
    CaptchaImage {
        captcha_on_off: true,
        uuid,
        img: captcha.base_img,
    }
}

fn get_file_type(content_type: &str) -> String {
    match content_type {
        "image/jpeg" => ".jpg".to_string(),
        "image/png" => ".png".to_string(),
        "image/gif" => ".gif".to_string(),
        _ => "".to_string(),
    }
}

/// 上传相关
pub async fn upload_file(mut multipart: Multipart, old_path: Option<&str>) -> Result<String> {
    if let Some(field) = multipart.next_field().await? {
        let content_type = field.content_type().unwrap_or("");
        let file_type = get_file_type(content_type);
        let bytes = field.bytes().await?;
        let now = chrono::Local::now();
        let file_dir = CFG.web.upload_dir.clone() + "/" + &now.format("%Y-%m").to_string();
        fs::create_dir_all(file_dir.clone()).await?;
        let file_name =
            now.format("%d").to_string() + "-" + &scru128::scru128_string() + &file_type;
        let path = CFG.web.dir.clone() + "/" + &file_dir + "/" + &file_name;
        let mut file = fs::File::create(&path).await?;
        file.write_all(&bytes).await?;
        Ok(file_dir + "/" + &file_name)
    } else {
        Err(anyhow::anyhow!("上传文件失败"))
    }
}
