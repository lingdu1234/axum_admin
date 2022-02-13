use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SearchReq {
    pub title: Option<String>,
    pub oper_name: Option<String>,
    pub operator_type: Option<String>,
    pub status: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize)]
pub struct DeleteReq {
    pub oper_log_ids: Vec<String>,
}
