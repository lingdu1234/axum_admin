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

#[derive(Debug, Serialize)]
/// 增 删 改 数据返回
pub struct CudResData<T> {
    pub id: Option<T>,
    pub msg: String,
}

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct PageParams {
    pub page_num: Option<usize>,
    pub page_size: Option<usize>,
}

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct RespData {
    pub code: Option<i32>,
    #[serde(flatten)]
    pub data: Option<serde_json::Value>,
    pub msg: Option<String>,
}

impl RespData {
    pub fn with_data(data: serde_json::Value) -> Self {
        Self {
            code: Some(200),
            data: Some(data),
            msg: Some("success".to_string()),
        }
    }
    pub fn with_msg(msg: &str) -> Self {
        Self {
            code: Some(200),
            data: None,
            msg: Some(msg.to_string()),
        }
    }
    pub fn with_err(err: &str) -> Self {
        Self {
            code: Some(500),
            data: None,
            msg: Some(err.to_string()),
        }
    }
    pub fn new(data: serde_json::Value, msg: &str) -> Self {
        Self {
            code: Some(200),
            data: Some(data),
            msg: Some(msg.to_string()),
        }
    }
}

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
    pub fn with_msg(msg: &str) -> Self {
        Self {
            code: Some(200),
            data: None,
            msg: Some(msg.to_string()),
        }
    }
    pub fn with_data_msg(data: Option<T>, msg: &str) -> Self {
        Self {
            code: Some(200),
            data,
            msg: Some(msg.to_string()),
        }
    }
    pub fn with_err(err: &str) -> Self {
        Self {
            code: Some(500),
            data: None,
            msg: Some(err.to_string()),
        }
    }
    pub fn new(code: i32, data: T, msg: &str) -> Self {
        Self {
            code: Some(code),
            data: Some(data),
            msg: Some(msg.to_string()),
        }
    }
}
