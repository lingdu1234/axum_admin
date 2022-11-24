mod task;
mod task_builder;
mod task_runner;

use std::{collections::HashMap, sync::Arc};

use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use db::{db_conn, system::SysJobModel, DB};
use once_cell::sync::Lazy;
pub use task_builder::{build_task, TASK_TIMER};
pub use task_runner::{delete_job, get_next_task_run_time, get_task_end_time, run_once_task};
use tokio::{
    sync::Mutex,
    time::{sleep, Duration},
};

use super::system;

pub static TASK_MODELS: Lazy<Arc<Mutex<HashMap<i64, TaskModel>>>> = Lazy::new(|| {
    let tasks: HashMap<i64, TaskModel> = HashMap::new();
    Arc::new(Mutex::new(tasks))
});

#[derive(Debug, Clone)]
pub struct TaskModel {
    pub run_lot: i64,
    pub count: i64,
    pub lot_count: i64,
    pub next_run_time: NaiveDateTime,
    pub lot_end_time: NaiveDateTime,
    pub model: SysJobModel,
}

pub async fn timer_task_init() -> Result<()> {
    // 获取任务列表
    let db = DB.get_or_init(db_conn).await;
    let task_list = match system::get_active_job(db).await {
        Ok(x) => x,
        Err(e) => return Err(anyhow!("{:#?}", e)),
    };
    // 初始化任务
    for t in task_list {
        // match t.task_count {
        // 0..=99 => {
        task_runner::add_circles_task(t.clone()).await?;
        sleep(Duration::from_secs(1)).await;
        //     }
        //     _ => {
        //         tokio::spawn(async move {
        //             timer_big_task_init(t.clone())
        //                 .await
        //                 .expect("任务初始化失败");
        //         });
        //     }
        // };
    }
    Ok(())
}

// pub async fn timer_big_task_init(t: system::SysJobModel) -> Result<()> {
//     let mut tt = t.clone();
//     tt.task_count = 99;
//     task_runner::add_circles_task(tt.clone()).await?;
//     sleep(Duration::from_secs(5)).await;
//     update_circles_task(t.job_id.clone())
//         .await
//         .expect("更新任务失败");
//     Ok(())
// }

pub async fn run_circles_task(job_id: String) -> Result<()> {
    let db = DB.get_or_init(db_conn).await;
    let t = match system::get_job_by_id(db, job_id).await {
        Ok(x) => x,
        Err(e) => return Err(anyhow!("{:#?}", e)),
    };
    task_runner::add_circles_task(t).await?;
    Ok(())
}

pub async fn update_circles_task(job_id: String) -> Result<()> {
    let db = DB.get_or_init(db_conn).await;
    let t = match system::get_job_by_id(db, job_id).await {
        Ok(x) => x,
        Err(e) => return Err(anyhow!("{:#?}", e)),
    };
    tracing::info!("开始更新任务 {}", &t.job_name);
    task_runner::update_circles_task(t).await?;
    Ok(())
}
