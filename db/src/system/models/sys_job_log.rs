use sea_orm::entity::prelude::DateTime;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SysJobLogSearchReq {
    pub job_log_id: Option<String>,
    pub job_id: Option<String>,
    pub job_name: Option<String>,
    pub job_group: Option<String>,
    pub is_once: Option<String>,
    pub status: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct SysJobLogAddReq {
    pub job_id: String,
    pub job_name: String,
    pub job_group: String,
    pub invoke_target: String,
    pub job_params: Option<String>,
    pub job_message: Option<String>,
    pub exception_info: Option<String>,
    pub status: String,
    pub created_at: DateTime,
    pub elapsed_time: i64,
    pub lot_id: i64,
    pub lot_order: i64,
    pub is_once: Option<String>,
}

#[derive(Deserialize)]
pub struct SysJobLogDeleteReq {
    pub job_log_ids: Vec<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SysJobLogCleanReq {
    pub job_id: String,
}
