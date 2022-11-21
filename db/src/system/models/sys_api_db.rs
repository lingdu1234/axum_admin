use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct SysApiDbAddEditReq {
    pub api_id: String,
    pub dbs: Vec<String>,
}

#[derive(Deserialize)]
pub struct SysApiDbSearchReq {
    pub api_id: String,
}
