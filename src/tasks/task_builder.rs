use super::tasks::run_task;
use delay_timer::{
    anyhow::Result, entity::DelayTimer, prelude::*, utils::convenience::async_template,
};
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::Mutex;

pub static TASK_TIMER: Lazy<Arc<Mutex<DelayTimer>>> = Lazy::new(|| {
    let t_timeer = DelayTimerBuilder::default()
        .tokio_runtime_by_default()
        .build();
    Arc::new(Mutex::new(t_timeer))
});

pub fn build_task(
    job_id: &str,
    cron_str: &str,
    task_name: &str,
    task_count: u64,
    task_id: u64,
    params: Option<String>,
) -> Result<Task> {
    return build_task_async_task(job_id, cron_str, task_name, task_count, task_id, params);
}

fn build_task_async_task(
    job_id: &str,
    cron_str: &str,
    task_name: &str,
    task_count: u64,
    task_id: u64,
    params: Option<String>,
) -> Result<Task> {
    let mut task_builder = TaskBuilder::default();
    let t_name = task_name.to_string();
    let j_id = job_id.to_string();
    let body = move || {
        let tt_name = t_name.clone();
        let pp = params.clone();
        let jj_id = j_id.clone();
        generate_closure_template(jj_id, tt_name)
    };
    let task = match task_count {
        0 => task_builder
            .set_frequency_repeated_by_cron_str(cron_str)
            .set_task_id(task_id)
            .spawn_async_routine(body)?,
        x => task_builder
            .set_frequency_count_down_by_cron_str(cron_str, x)
            .set_task_id(task_id)
            .spawn_async_routine(body)?,
    };
    Ok(task)
}

async fn generate_closure_template(job_id: String, task_name: String) {
    let t = task_name.to_string();
    let future_inner = async_template(get_timestamp() as i32, t.clone());
    run_task(job_id).await;
    future_inner.await.ok();
}
