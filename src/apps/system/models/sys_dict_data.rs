use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Debug, Serialize, Default, Validate)]
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

#[derive(Deserialize, Clone, Debug, Serialize, Default, Validate)]
pub struct AddReq {
    pub dict_type: String,
    pub dict_label: String,
    pub dict_value: String,
    pub dict_sort: i32,
    #[validate(range(min = 0, max = 1))]
    pub is_default: i8,
    #[validate(range(min = 0, max = 1))]
    pub status: Option<i8>,
    #[validate(length(min = 1))]
    pub remark: Option<String>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct DeleteReq {
    pub dict_ids: Vec<String>,
}

#[derive(Deserialize, Debug, Serialize, Validate)]
pub struct EditReq {
    pub dict_data_id: String,
    pub dict_type: String,
    pub dict_label: String,
    pub dict_value: String,
    pub dict_sort: i32,
    #[validate(range(min = 0, max = 1))]
    pub is_default: i8,
    #[validate(range(min = 0, max = 1))]
    pub status: i8,
    pub remark: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Resp {
    pub dict_type: String,
    pub dict_label: String,
    pub dict_value: String,
    pub dict_sort: i32,
    pub is_default: i8,
    pub status: i8,
    pub remark: String,
    pub created_at: String,
}
