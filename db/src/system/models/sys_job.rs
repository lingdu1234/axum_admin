use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Debug)]
pub struct SysJobSearchReq {
    pub job_id: Option<String>,
    pub job_name: Option<String>,
    pub job_group: Option<String>,
    pub status: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SysJobAddReq {
    pub task_id: i64,
    pub task_count: i64,
    pub job_name: String,
    pub job_params: Option<String>,
    pub job_group: String,
    pub invoke_target: String,
    pub cron_expression: String,
    pub misfire_policy: String,
    pub concurrent: Option<String>,
    pub status: Option<String>,
    pub remark: Option<String>,
}

#[derive(Deserialize)]
pub struct SysJobDeleteReq {
    pub job_ids: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct SysJobEditReq {
    pub job_id: String,
    pub task_id: i64,
    pub task_count: i64,
    pub job_name: String,
    pub job_params: Option<String>,
    pub job_group: String,
    pub invoke_target: String,
    pub cron_expression: String,
    pub misfire_policy: String,
    pub concurrent: Option<String>,
    pub status: Option<String>,
    pub remark: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SysJobStatusReq {
    pub job_id: String,
    pub status: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct JobId {
    pub job_id: String,
    pub task_id: i64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ValidateReq {
    pub cron_str: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct ValidateRes {
    pub validate: bool,
    pub next_ten: Option<Vec<NaiveDateTime>>,
}
