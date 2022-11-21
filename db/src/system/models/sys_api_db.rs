use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, Clone,ToSchema)]
pub struct SysApiDbAddEditReq {
    pub api_id: String,
    pub dbs: Vec<String>,
}

#[derive(Deserialize,ToSchema)]
pub struct SysApiDbSearchReq {
    pub api_id: String,
}
