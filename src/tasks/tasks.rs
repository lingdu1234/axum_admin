mod tests;

use crate::{
    apps::system::{get_job_by_id, sys_job_log_add, SysJobLogAddReq, SysJobModel},
    database::{db_conn, DB},
};
use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDateTime};

/// 此处配置任务名称，用于前端添加测试名称，用于调用任务函数
fn go_run_task(params: Option<String>, task_name: String) -> Result<String> {
    match task_name.as_str() {
        "test_a" => tests::test_a(params),
        "test_b" => tests::test_b(params),
        "test_c" => tests::test_b(params),
        _ => Err(anyhow!("任务 {} 未找到", task_name)),
    }
}

pub async fn run_task(job_id: String) {
    let db = DB.get_or_init(db_conn).await;
    let job = get_job_by_id(db, job_id.clone())
        .await
        .expect("job not found");
    let begin_time = Local::now().naive_local();
    let res = go_run_task(job.job_params.clone(), job.job_name.clone());
    let end_time = Local::now().naive_local(); //获取结束时间
    let elapsed_time: i64 = end_time
        .signed_duration_since(begin_time)
        .to_std()
        .unwrap()
        .as_millis()
        .try_into()
        .unwrap_or_else(|_| i64::MAX); //获取结束时间和开始时间的时间差，单位为毫秒
    tokio::spawn(async move {
        write_job_log(res, job, begin_time, elapsed_time).await;
    });
}

async fn write_job_log(
    res: Result<String>,
    job: SysJobModel,
    begin_time: NaiveDateTime,
    elapsed_time: i64,
) {
    let (job_message, exception_info, status) = match res {
        Ok(x) => (Some(x), None, "1".to_string()),
        Err(e) => (None, Some(format!("{:#?}", e)), "0".to_string()),
    };
    let job_log = SysJobLogAddReq {
        job_id: job.job_id.clone(),
        job_name: job.job_name.clone(),
        job_group: job.job_group.clone(),
        invoke_target: job.invoke_target.clone(),
        job_params: job.job_params.clone(),
        job_message,
        exception_info,
        status,
        created_at: begin_time,
        elapsed_time,
    };
    let db = DB.get_or_init(db_conn).await;
    sys_job_log_add(db, job_log)
        .await
        .expect("write job log failed");
}
