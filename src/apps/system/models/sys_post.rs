use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Debug, Serialize, Default, Validate)]
pub struct SearchReq {
    pub post_id: Option<String>,
    #[validate(length(min = 1))]
    pub post_code: Option<String>,
    #[validate(length(min = 1))]
    pub post_name: Option<String>,
    #[validate(range(min = 0, max = 2))]
    pub status: Option<i8>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize, Clone, Debug, Serialize, Default, Validate)]
pub struct AddReq {
    pub post_code: String,
    pub post_name: String,
    pub post_sort: i32,
    #[validate(range(min = 0, max = 1))]
    pub status: Option<i8>,
    #[validate(length(min = 1))]
    pub remark: Option<String>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct DeleteReq {
    #[validate(length(min = 1))]
    pub post_ids: Vec<String>,
}

#[derive(Deserialize, Clone, Debug, Serialize, Validate)]
pub struct EditReq {
    pub post_id: String,
    pub post_code: String,
    pub post_name: String,
    pub post_sort: i32,
    #[validate(range(min = 0, max = 1))]
    pub status: i8,
    pub remark: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Resp {
    pub post_id: String,
    pub post_code: String,
    pub post_name: String,
    pub post_sort: i32,
    pub status: i8,
    pub remark: String,
    pub created_at: String,
}
