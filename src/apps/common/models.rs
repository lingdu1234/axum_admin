use poem::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CaptchaImage {
    pub captcha_on_off: bool,
    pub uuid: String,
    pub img: String,
}

#[derive(Debug, Serialize)]
/// 查 数据返回
pub struct ListData<T> {
    pub list: Vec<T>,
    pub total: usize,
    pub total_pages: usize,
    pub page_num: usize,
}

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct PageParams {
    pub page_num: Option<usize>,
    pub page_size: Option<usize>,
}

/// 数据统一返回格式
#[derive(Deserialize, Debug, Serialize, Default)]
pub struct Res<T> {
    pub code: Option<i32>,
    pub data: Option<T>,
    pub msg: Option<String>,
}

impl IntoResponse for Res<serde_json::Value> {
    fn into_response(self) -> Response {
        let data = Self {
            code: self.code,
            data: self.data,
            msg: self.msg,
        };
        data.into_response()
    }
}

impl<T: Serialize> Res<T> {
    pub fn with_data(data: T) -> Self {
        Self {
            code: Some(200),
            data: Some(data),
            msg: Some("success".to_string()),
        }
    }
    pub fn with_err(err: &str) -> Self {
        Self {
            code: Some(500),
            data: None,
            msg: Some(err.to_string()),
        }
    }
    pub fn with_msg(msg: &str) -> Self {
        Self {
            code: Some(200),
            data: None,
            msg: Some(msg.to_string()),
        }
    }
    #[allow(dead_code)]
    pub fn with_data_msg(data: Option<T>, msg: &str) -> Self {
        Self {
            code: Some(200),
            data,
            msg: Some(msg.to_string()),
        }
    }
}
