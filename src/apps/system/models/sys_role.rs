use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Debug, Serialize, Default, Validate)]
pub struct SearchReq {
    pub id: Option<String>,
    #[validate(length(min = 1))]
    pub name: Option<String>,
    #[validate(range(min = 0, max = 2))]
    pub status: Option<i8>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize, Clone, Debug, Serialize, Default, Validate)]
pub struct AddReq {
    pub name: String,
    pub list_order: i32,
    #[validate(range(min = 0, max = 5))]
    pub data_scope: i8,
    #[validate(range(min = 0, max = 1))]
    pub status: Option<i8>,
    #[validate(length(min = 1))]
    pub remark: Option<String>,
    pub menu_ids: Vec<String>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct DeleteReq {
    pub role_ids: Vec<String>,
}

#[derive(Deserialize, Clone, Debug, Serialize, Validate)]
pub struct EditReq {
    pub id: String,
    pub name: String,
    pub list_order: i32,
    #[validate(range(min = 0, max = 5))]
    pub data_scope: i8,
    #[validate(range(min = 0, max = 1))]
    pub status: i8,
    #[validate(length(min = 1))]
    pub remark: String,
    pub menu_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate, FromQueryResult, Clone)]
pub struct Resp {
    pub id: String,
    pub status: i8,
    pub list_order: i32,
    pub name: String,
    pub remark: String,
    pub data_scope: i8,
}
