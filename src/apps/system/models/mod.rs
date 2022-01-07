use poem::{error::IntoResult, web::Json, IntoResponse, Response};
use serde::{Deserialize, Serialize};

pub mod common;
pub mod sys_dept;
pub mod sys_dict_data;
pub mod sys_dict_type;
pub mod sys_menu;
pub mod sys_post;
pub mod sys_role;
pub mod sys_user;

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
            data: data,
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
