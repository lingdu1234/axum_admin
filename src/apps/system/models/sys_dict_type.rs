use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct SearchReq {
    pub dict_name: Option<String>,
    pub dict_type: Option<String>,
    pub status: Option<i8>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct AddReq {
    pub dict_name: Option<String>,
    pub dict_type: Option<String>,
    pub status: Option<i8>,
    pub remark: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct EditReq {
    pub dict_id: String,
    pub dict_name: Option<String>,
    pub dict_type: Option<String>,
    pub status: Option<i8>,
    pub remark: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct Resp {
    pub dict_id: String,
    pub dict_name: Option<String>,
    pub dict_type: Option<String>,
    pub status: Option<i8>,
    pub remark: Option<String>,
}
