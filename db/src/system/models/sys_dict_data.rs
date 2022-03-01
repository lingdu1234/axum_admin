use sea_orm::{entity::prelude::DateTime, FromQueryResult};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct SearchReq {
    pub dict_data_id: Option<String>,
    #[validate(length(min = 1))]
    pub dict_type: Option<String>,
    #[validate(length(min = 1))]
    pub dict_label: Option<String>,
    #[validate(range(min = 0, max = 2))]
    pub status: Option<i8>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Default, Deserialize, Clone, Debug, Validate)]
pub struct AddReq {
    pub dict_type: String,
    pub dict_label: String,
    pub dict_value: String,
    pub dict_sort: i32,
    pub css_class: Option<String>,
    pub list_class: Option<String>,
    #[validate(length(min = 1))]
    pub is_default: String,
    #[validate(length(min = 1))]
    pub status: Option<String>,
    #[validate(length(min = 1))]
    pub remark: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct DeleteReq {
    pub dict_data_ids: Vec<String>,
}

#[derive(Deserialize, Debug, Validate)]
pub struct EditReq {
    pub dict_data_id: String,
    pub dict_type: String,
    pub dict_label: String,
    pub dict_value: String,
    pub dict_sort: i32,
    pub css_class: Option<String>,
    pub list_class: Option<String>,
    pub is_default: String,
    pub status: String,
    pub remark: String,
}

#[derive(Debug, Serialize, Validate, FromQueryResult, Clone)]
pub struct Resp {
    pub dict_type: String,
    pub dict_label: String,
    pub dict_value: String,
    pub dict_sort: i32,
    pub is_default: String,
    pub status: String,
    pub remark: String,
    pub created_at: DateTime,
}
