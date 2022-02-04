mod task_builder;
mod tasks;
use crate::{
    apps::system,
    database::{db_conn, DB},
};
use anyhow::{anyhow, Result};

pub use task_builder::{build_task, TASK_TIMER};
pub use tasks::run_task as run_once_task;
use tracing::info;

pub async fn timer_task_init() -> Result<()> {
    // 获取任务列表
    let db = DB.get_or_init(db_conn).await;
    let task_list = match system::get_active_job(db).await {
        Ok(x) => x,
        Err(e) => return Err(anyhow!("{:#?}", e)),
    };
    // 初始化任务
    for t in task_list {
        add_circles_task(t).await?;
    }
    Ok(())
}

async fn add_circles_task(t: system::SysJobModel) -> Result<()> {
    let t_builder = task_builder::TASK_TIMER.lock().await;
    let task = task_builder::build_task(
        &t.job_id,
        &t.cron_expression,
        &t.job_name,
        t.task_count.try_into().unwrap_or(0),
        t.task_id.try_into().unwrap_or(0),
        t.job_params.clone(),
    );
    match task {
        Ok(x) => {
            match t_builder.add_task(x) {
                Ok(_) => {}
                Err(e) => return Err(anyhow!("{:#?}", e)),
            };
        }
        Err(e) => return Err(anyhow!("{:#?}", e)),
    };
    Ok(())
}

pub async fn run_circles_task(job_id: String) -> Result<()> {
    let db = DB.get_or_init(db_conn).await;
    let t = match system::get_job_by_id(db, job_id).await {
        Ok(x) => x,
        Err(e) => return Err(anyhow!("{:#?}", e)),
    };
    add_circles_task(t).await?;
    Ok(())
}
