use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SysUserOnlineSearchReq {
    pub ipaddr: Option<String>,
    pub user_name: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SysUserOnlineDeleteReq {
    pub ids: Vec<String>,
}
