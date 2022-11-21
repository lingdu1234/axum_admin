use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct SysLoginLogSearchReq {
    pub ip: Option<String>,
    pub user_name: Option<String>,
    pub status: Option<String>,
    pub order_by_column: Option<String>,
    pub is_asc: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SysLoginLogDeleteReq {
    pub info_ids: Vec<String>,
}
