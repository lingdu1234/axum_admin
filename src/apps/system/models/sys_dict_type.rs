use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct SearchReq {
    pub dict_name: Option<String>,
    pub dict_type: Option<String>,
    pub status: Option<i8>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}
