use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, Debug,ToSchema)]
pub struct SysUserOnlineSearchReq {
    pub ipaddr: Option<String>,
    pub user_name: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Debug, Deserialize,ToSchema)]
pub struct SysUserOnlineDeleteReq {
    pub ids: Vec<String>,
}
