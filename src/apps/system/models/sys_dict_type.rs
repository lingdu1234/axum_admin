use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use validator::Validate;
#[derive(Deserialize, Debug, Serialize, Default, Validate)]
pub struct SearchReq {
    pub dict_type_id: Option<String>,
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

#[derive(Deserialize, Serialize, Validate)]
pub struct DeleteReq {
    pub dict_type_ids: Vec<String>,
}

#[derive(Deserialize, Debug, Serialize, Default, Validate)]
pub struct EditReq {
    pub dict_type_id: String,
    pub dict_name: String,
    pub dict_type: String,
    pub status: i8,
    pub remark: String,
}

#[derive(Deserialize, Debug, Serialize, FromQueryResult, Validate)]
pub struct Resp {
    pub dict_type_id: String,
    pub dict_name: String,
    pub dict_type: String,
    pub status: i8,
    pub remark: String,
}
