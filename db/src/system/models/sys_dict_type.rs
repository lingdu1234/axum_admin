use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use validator::Validate;
#[derive(Deserialize, Debug, Validate)]
pub struct SearchReq {
    pub dict_type_id: Option<String>,
    pub dict_name: Option<String>,
    pub dict_type: Option<String>,
    pub status: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize, Debug, Validate)]
pub struct AddReq {
    pub dict_name: String,
    pub dict_type: String,
    pub status: Option<String>,
    #[validate(length(min = 1))]
    pub remark: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct DeleteReq {
    pub dict_type_ids: Vec<String>,
}

#[derive(Deserialize, Debug, Validate)]
pub struct EditReq {
    pub dict_type_id: String,
    pub dict_name: String,
    pub dict_type: String,
    pub status: String,
    pub remark: String,
}

#[derive(Debug, Serialize, FromQueryResult, Validate)]
pub struct Resp {
    pub dict_type_id: String,
    pub dict_name: String,
    pub dict_type: String,
    pub status: String,
    pub remark: String,
}
