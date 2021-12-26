use serde::{Deserialize, Serialize};

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
    pub data: Option<serde_json::Value>,
    pub msg: Option<String>,
}

impl RespData {
    pub fn with_data(data: serde_json::Value) -> Self {
        Self {
            data: Some(data),
            msg: Some("success".to_string()),
        }
    }
    pub fn with_msg(msg: &str) -> Self {
        Self {
            data: None,
            msg: Some(msg.to_string()),
        }
    }
    pub fn new(data: serde_json::Value, msg: &str) -> Self {
        Self {
            data: Some(data),
            msg: Some(msg.to_string()),
        }
    }
}
