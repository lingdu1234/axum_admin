use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, Clone, Debug,ToSchema)]
pub struct SysUpdateLogAddReq {
    pub app_version: String,
    pub backend_version: String,
    pub title: String,
    pub content: String,
}

#[derive(Deserialize, Clone, Debug,ToSchema)]
pub struct SysUpdateLogEditReq {
    pub id: String,
    pub app_version: String,
    pub backend_version: String,
    pub title: String,
    pub content: String,
}

#[derive(Deserialize,ToSchema)]
pub struct SysUpdateLogDeleteReq {
    pub id: String,
}
