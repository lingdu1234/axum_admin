use sea_orm::{entity::prelude::DateTime, FromQueryResult};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct SearchReq {
    pub post_id: Option<String>,
    #[validate(length(min = 1))]
    pub post_code: Option<String>,
    #[validate(length(min = 1))]
    pub post_name: Option<String>,
    pub status: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize, Clone, Debug, Validate)]
pub struct AddReq {
    pub post_code: String,
    pub post_name: String,
    pub post_sort: i32,
    pub status: Option<String>,
    #[validate(length(min = 1))]
    pub remark: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct DeleteReq {
    #[validate(length(min = 1))]
    pub post_ids: Vec<String>,
}

#[derive(Deserialize, Clone, Debug, Validate)]
pub struct EditReq {
    pub post_id: String,
    pub post_code: String,
    pub post_name: String,
    pub post_sort: i32,
    pub status: String,
    pub remark: String,
}

#[derive(Debug, Serialize, Validate, FromQueryResult)]
pub struct Resp {
    pub post_id: String,
    pub post_code: String,
    pub post_name: String,
    pub post_sort: i32,
    pub status: String,
    pub remark: String,
    pub created_at: DateTime,
}
