use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct SearchReq {
    pub ipaddr: Option<String>,
    pub user_name: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct DeleteReq {
    #[validate(length(min = 1, message = "至少要有一个id"))]
    pub ids: Vec<String>,
}
