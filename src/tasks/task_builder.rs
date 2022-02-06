use super::run_once_task;
use delay_timer::{
    anyhow::Result,
    entity::DelayTimer,
    prelude::*,
    timer::task::{ScheduleIteratorTimeZone::Local, Task},
    utils::convenience::async_template,
};
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::Mutex;

pub static TASK_TIMER: Lazy<Arc<Mutex<DelayTimer>>> = Lazy::new(|| {
    let t_timeer = DelayTimerBuilder::default()
        // .enable_status_report()
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
) -> Result<Task> {
    return build_task_async_task(job_id, cron_str, task_name, task_count, task_id);
}
fn build_task_async_task(
    job_id: &str,
    cron_str: &str,
    job_name: &str,
    task_count: u64,
    task_id: u64,
) -> Result<Task> {
    let mut task_builder = TaskBuilder::default();
    task_builder.set_schedule_iterator_time_zone(Local);
    let t_name = job_name.to_string();
    let j_id = job_id.to_string();
    let body = move || {
        let tt_name = t_name.clone();
        let jj_id = j_id.clone();
        generate_closure_template(jj_id, task_id, tt_name)
    };
    let task = task_builder
        .set_frequency_count_down_by_cron_str(cron_str, task_count)
        .set_task_id(task_id)
        .spawn_async_routine(body)?;
    Ok(task)
}

async fn generate_closure_template(job_id: String, task_id: u64, job_name: String) {
    let t = job_name.to_string();
    let t_id: i64 = task_id as i64;
    let future_inner = async_template(get_timestamp() as i32, t.clone());
    run_once_task(job_id, t_id, false).await;
    future_inner.await.ok();
}
