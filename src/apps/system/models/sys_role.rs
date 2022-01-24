use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Debug, Serialize, Default, Validate)]
pub struct SearchReq {
    pub role_id: Option<String>,
    #[validate(length(min = 1))]
    pub name: Option<String>,
    pub status: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize, Clone, Debug, Serialize, Default, Validate)]
pub struct AddReq {
    pub role_name: String,
    pub role_key: String,
    pub list_order: i32,
    pub data_scope: Option<String>,
    pub status: Option<String>,
    #[validate(length(min = 1))]
    pub remark: Option<String>,
    pub menu_ids: Vec<String>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct DeleteReq {
    pub role_ids: Vec<String>,
}
#[derive(Deserialize, Serialize, Validate)]
pub struct DataScopeReq {
    pub role_id: String,
    pub data_scope: String,
    pub dept_ids: Vec<String>,
}

#[derive(Deserialize, Clone, Debug, Serialize, Validate)]
pub struct EditReq {
    pub role_id: String,
    pub role_name: String,
    pub role_key: String,
    pub list_order: i32,
    pub data_scope: String,
    pub status: String,
    pub remark: String,
    pub menu_ids: Vec<String>,
}
#[derive(Deserialize, Clone, Debug, Serialize, Validate)]
pub struct StatusReq {
    pub role_id: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, FromQueryResult, Clone)]
pub struct Resp {
    pub role_id: String,
    pub role_name: String,
    pub role_key: String,
    pub status: String,
    pub list_order: i32,
    pub remark: String,
    pub data_scope: String,
}
