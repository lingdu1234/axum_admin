use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct SearchReq {
    pub ip: Option<String>,
    pub user_name: Option<String>,
    pub status: Option<String>,
    pub order_by_column: Option<String>,
    pub is_asc: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct DeleteReq {
    #[validate(length(min = 1, message = "至少要有一个id"))]
    pub info_ids: Vec<String>,
}
