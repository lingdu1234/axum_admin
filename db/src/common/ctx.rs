use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct ReqCtx {
    pub ori_uri: String,
    pub path: String,
    pub method: String,
    pub user: UserInfo,
    pub data: String,
}

#[derive(Debug, Clone)]
pub struct UserInfo {
    pub id: String,
    pub token_id: String,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct ResInfo {
    pub duration: String,
    pub status: String,
    pub data: String,
    pub err_msg: String,
}
