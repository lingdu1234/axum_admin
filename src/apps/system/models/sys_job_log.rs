use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SearchReq {
    pub job_log_id: Option<String>,
    pub job_id: Option<String>,
    pub job_name: Option<String>,
    pub job_group: Option<String>,
    pub status: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct AddReq {
    pub job_id: String,
    pub job_name: String,
    pub job_group: String,
    pub invoke_target: String,
    pub job_params: Option<String>,
    pub job_message: Option<String>,
    pub exception_info: Option<String>,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub elapsed_time: i64,
}

#[derive(Deserialize)]
pub struct DeleteReq {
    pub job_log_ids: Vec<String>,
}
