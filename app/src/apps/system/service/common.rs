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
pub async fn upload_file(mut multipart: Multipart) -> Result<String> {
    if let Some(field) = multipart.next_field().await? {
        let content_type = field.content_type().map(ToString::to_string).unwrap_or_else(|| "".to_string());
        let old_url = field.file_name().map(ToString::to_string).unwrap_or_else(|| "".to_string());
        let file_type = get_file_type(&content_type);
        let bytes = field.bytes().await?;
        let now = chrono::Local::now();
        let file_path_t = CFG.web.upload_dir.clone() + "/" + &now.format("%Y-%m").to_string();
        let url_path_t = CFG.web.upload_url.clone() + "/" + &now.format("%Y-%m").to_string();
        fs::create_dir_all(&file_path_t).await?;
        let file_name = now.format("%d").to_string() + "-" + &scru128::new_string() + &file_type;
        let file_path = file_path_t + "/" + &file_name;
        let url_path = url_path_t + "/" + &file_name;
        let mut file = fs::File::create(&file_path).await?;
        file.write_all(&bytes).await?;
        if !old_url.is_empty() {
            self::delete_file(&old_url).await;
        }
        Ok(url_path)
    } else {
        Err(anyhow::anyhow!("上传文件失败"))
    }
}

/// 删除文件
pub async fn delete_file(file_path: &str) {
    let path = file_path.replace(&CFG.web.upload_url, &CFG.web.upload_dir);
    match fs::remove_file(&path).await {
        Ok(_) => {}
        Err(_) => {
            tracing::error!("删除文件失败:{}", path);
        }
    }
}
