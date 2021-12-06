use serde::{Deserialize, Serialize};
use validator::Validate;
#[derive(Deserialize, Debug, Serialize, Default, Validate)]
pub struct SearchReq {
    pub dict_id: Option<String>,
    #[validate(length(min = 1))]
    pub dict_name: Option<String>,
    #[validate(length(min = 1))]
    pub dict_type: Option<String>,
    #[validate(range(min = 0, max = 2))]
    pub status: Option<i8>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Default, Validate)]
pub struct AddReq {
    pub dict_name: String,
    pub dict_type: String,
    #[validate(range(min = 0, max = 2))]
    pub status: Option<i8>,
    #[validate(length(min = 1))]
    pub remark: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct DeleteReq {
    pub dict_ids: Vec<String>,
}

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct EditReq {
    pub dict_id: String,
    pub dict_name: String,
    pub dict_type: String,
    pub status: i8,
    pub remark: String,
}

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct Resp {
    pub dict_id: String,
    pub dict_name: String,
    pub dict_type: String,
    pub status: i8,
    pub remark: String,
}
