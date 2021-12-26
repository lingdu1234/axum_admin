use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Debug, Serialize, Default, Validate)]
pub struct SearchReq {
    pub dept_id: Option<String>,
    #[validate(length(min = 1))]
    pub dept_name: Option<String>,
    #[validate(range(min = 0, max = 2))]
    pub status: Option<i8>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize, Clone, Debug, Serialize, Default, Validate)]
pub struct AddReq {
    pub parent_id: String,
    pub dept_name: String,
    pub order_num: i32,
    pub leader: String,
    pub phone: String,
    pub email: String,
    pub status: i8,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct DeleteReq {
    #[validate(length(min = 1))]
    pub dept_ids: Vec<String>,
}

#[derive(Deserialize, Clone, Debug, Serialize, Validate)]
pub struct EditReq {
    pub dept_id: String,
    pub parent_id: String,
    pub dept_name: String,
    pub order_num: i32,
    pub leader: String,
    pub phone: String,
    pub email: String,
    pub status: i8,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, FromQueryResult, Default)]
pub struct Resp {
    pub dept_id: String,
    pub parent_id: String,
    pub dept_name: String,
    pub order_num: i32,
    pub leader: String,
    pub phone: String,
    pub email: String,
    pub status: i8,
    pub created_at: String,
}

#[derive(Deserialize, Clone, Debug, Serialize, Validate, Default)]
pub struct RespTree {
    #[serde(flatten)]
    pub data: Resp,
    pub children: Option<Vec<RespTree>>,
}
