// use crate::RT;

use std::sync::Arc;

use delay_timer::{
    anyhow::Result,
    entity::DelayTimer,
    prelude::*,
    timer::task::{ScheduleIteratorTimeZone::Utc, Task},
    utils::convenience::async_template,
};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;
use utils::my_env::RT;

use super::run_once_task;

pub static TASK_TIMER: Lazy<Arc<RwLock<DelayTimer>>> = Lazy::new(|| {
    let rt = RT.clone();
    let t_timer = DelayTimerBuilder::default()
        // .enable_status_report()
        // .tokio_runtime_by_default()
        .tokio_runtime_shared_by_custom(rt)
        .build();
    Arc::new(RwLock::new(t_timer))
});
pub fn build_task(job_id: &str, cron_str: &str, task_name: &str, task_count: u64, task_id: u64) -> Result<Task> {
    build_task_async_task(job_id, cron_str, task_name, task_count, task_id)
}

fn build_task_async_task(job_id: &str, cron_str: &str, job_name: &str, task_count: u64, task_id: u64) -> Result<Task> {
    let mut task_builder = TaskBuilder::default();
    task_builder.set_schedule_iterator_time_zone(Utc);
    // .set_maximum_parallel_runnable_num(5);
    let t_name = job_name.to_string();
    let j_id = job_id.to_string();
    let body = move || {
        let tt_name = t_name.clone();
        let jj_id = j_id.clone();
        async move { generate_closure_template(jj_id, task_id, tt_name).await }
    };
    // let task = task_builder
    //     .set_frequency_count_down_by_cron_str(cron_str, task_count)
    //     .set_task_id(task_id)
    //     .spawn_async_routine(body)?;
    let task = match task_count {
        0 => task_builder.set_frequency_repeated_by_cron_str(cron_str).set_task_id(task_id).spawn_async_routine(body)?,
        x => task_builder
            .set_frequency_repeated_by_cron_str(cron_str)
            .set_task_id(task_id)
            .set_maximum_running_time(x)
            .spawn_async_routine(body)?,
    };
    Ok(task)
}

async fn generate_closure_template(job_id: String, task_id: u64, job_name: String) {
    let t = job_name.to_string();
    let t_id: i64 = task_id as i64;
    let future_inner = async_template(timestamp() as i32, t.clone());
    run_once_task(job_id, t_id, false).await;
    future_inner.await.ok();
}
