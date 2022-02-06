use std::str::FromStr;

use crate::{
    apps::system::{
        self, get_job_by_id, sys_job_log_add, SysJobColumn, SysJobEntity, SysJobLogAddReq,
        SysJobModel,
    },
    database::{db_conn, DB},
    utils,
};
use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDateTime};
use delay_timer::prelude::cron_clock;
use sea_orm::{sea_query::Expr, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};

use super::{task_builder, tasks, TaskModel, TASK_MODELS};

pub async fn run_once_task(job_id: String, task_id: i64, is_once: bool) {
    let mut task_models = TASK_MODELS.lock().await;
    let begin_time = Local::now().naive_local();
    let job = match is_once {
        true => {
            let db = DB.get_or_init(db_conn).await;
            let jm = get_job_by_id(db, job_id.clone())
                .await
                .expect("job not found");
            TaskModel {
                run_lot: "single task".to_string(),
                count: 0,
                lot_count: 0,
                next_run_time: begin_time,
                lot_end_time: begin_time,
                model: jm,
            }
        }
        false => task_models.get(&task_id).cloned().expect("task not found"),
    };
    let cron_str = job.model.cron_expression.clone();
    let next_time = get_next_task_run_time(cron_str).unwrap();
    let res = tasks::go_run_task(
        job.model.job_params.clone(),
        job.model.invoke_target.clone(),
    )
    .await;
    let task_end_time = Local::now().naive_local(); //获取结束时间
    let elapsed_time: i64 = task_end_time
        .signed_duration_since(begin_time)
        .to_std()
        .unwrap()
        .as_millis()
        .try_into()
        .unwrap_or_else(|_| i64::MAX); //获取结束时间和开始时间的时间差，单位为毫秒
    tokio::spawn(async move {
        match is_once {
            true => write_once_job_log(res, job, begin_time, elapsed_time).await,
            false => {
                task_models.entry(task_id).and_modify(|x| {
                    x.lot_count += 1;
                    x.next_run_time = next_time.clone();
                });
                let job_new = task_models.get(&task_id).cloned().expect("task not found");
                write_circle_job_log(res, job_new, begin_time, next_time, elapsed_time).await;
            }
        }
    });
}

pub async fn add_circles_task(t: system::SysJobModel) -> Result<()> {
    let task_count = match t.task_count {
        0 => i64::MAX,
        x => x,
    };

    let t_builder = task_builder::TASK_TIMER.lock().await;
    let task = task_builder::build_task(
        &t.job_id,
        &t.cron_expression,
        &t.job_name,
        task_count as u64,
        t.task_id.try_into().unwrap_or(0),
    );
    match task {
        Ok(x) => {
            init_task_model(t, task_count).await;
            match t_builder.add_task(x) {
                Ok(_) => {}
                Err(e) => return Err(anyhow!("{:#?}", e)),
            };
        }
        Err(e) => return Err(anyhow!("{:#?}", e)),
    };
    Ok(())
}

async fn init_task_model(m: SysJobModel, task_count: i64) {
    let job_end_time =
        get_task_end_time(m.cron_expression.clone(), m.task_count.try_into().unwrap()).unwrap();
    let next_time = get_next_task_run_time(m.cron_expression.clone()).unwrap();
    let mut task_models = TASK_MODELS.lock().await;
    let task_model = TaskModel {
        run_lot: utils::rand_s(10),
        count: task_count,
        lot_count: 0,
        next_run_time: next_time.clone(),
        lot_end_time: job_end_time,
        model: m.clone(),
    };
    task_models.insert(m.task_id.clone(), task_model);
    tokio::spawn(async move {
        let db = DB.get_or_init(db_conn).await;
        SysJobEntity::update_many()
            .col_expr(SysJobColumn::NextTime, Expr::value(next_time))
            .filter(SysJobColumn::JobId.eq(m.job_id.clone()))
            .exec(db)
            .await
            .expect("update job log failed");
    });
}

async fn write_circle_job_log(
    res: Result<String>,
    job: TaskModel,
    begin_time: NaiveDateTime,
    next_time: NaiveDateTime,
    elapsed_time: i64,
) {
    let (job_message, exception_info, status) = match res {
        Ok(x) => (Some(x), None, "1".to_string()),
        Err(e) => (None, Some(format!("{:#?}", e)), "0".to_string()),
    };
    let mut job_remark = "".to_string();
    let mut job_status = "1".to_string();
    //获取结束时间和开始时间的时间差，单位为毫秒
    match job.count as i64 == job.lot_count {
        false => {}
        true => {
            job_remark = format!(
                "批任务:[{}]已经执行完毕,完成时间:{}",
                job.run_lot.clone(),
                begin_time
            );
            job_status = "0".to_string();
        }
    };
    let job_log = SysJobLogAddReq {
        job_id: job.model.job_id.clone(),
        lot_id: job.run_lot.clone(),
        lot_order: job.lot_count,
        job_name: job.model.job_name.clone(),
        job_group: job.model.job_group.clone(),
        invoke_target: job.model.invoke_target.clone(),
        job_params: job.model.job_params.clone(),
        job_message,
        exception_info,
        status,
        is_once: Some("0".to_string()),
        created_at: begin_time,
        elapsed_time,
    };
    let db = DB.get_or_init(db_conn).await;
    let txn = db.begin().await.expect("事务开启失败");
    sys_job_log_add(&txn, job_log)
        .await
        .expect("write job log failed");

    SysJobEntity::update_many()
        .col_expr(SysJobColumn::Status, Expr::value(job_status))
        .col_expr(SysJobColumn::Remark, Expr::value(job_remark))
        .col_expr(SysJobColumn::LastTime, Expr::value(begin_time))
        .col_expr(SysJobColumn::NextTime, Expr::value(next_time))
        .filter(SysJobColumn::JobId.eq(job.model.job_id.clone()))
        .exec(&txn)
        .await
        .expect("update job log failed");
    txn.commit().await.expect("事务提交失败");
}

async fn write_once_job_log(
    res: Result<String>,
    job: TaskModel,
    begin_time: NaiveDateTime,
    elapsed_time: i64,
) {
    let (job_message, exception_info, status) = match res {
        Ok(x) => (Some(x), None, "1".to_string()),
        Err(e) => (None, Some(format!("{:#?}", e)), "0".to_string()),
    };

    let job_log = SysJobLogAddReq {
        job_id: job.model.job_id.clone(),
        lot_id: "单次任务".to_string(),
        lot_order: 0,
        job_name: job.model.job_name.clone(),
        job_group: job.model.job_group.clone(),
        invoke_target: job.model.invoke_target.clone(),
        job_params: job.model.job_params.clone(),
        job_message,
        exception_info,
        status,
        is_once: Some("1".to_string()),
        created_at: begin_time,
        elapsed_time,
    };
    let db = DB.get_or_init(db_conn).await;
    let txn = db.begin().await.expect("事务开启失败");
    sys_job_log_add(&txn, job_log)
        .await
        .expect("write job log failed");
    txn.commit().await.expect("事务提交失败");
}

pub fn get_next_task_run_time(cron_str: String) -> Option<NaiveDateTime> {
    let schedule = cron_clock::Schedule::from_str(&cron_str).unwrap();
    let next_time = match schedule.upcoming(Local).next() {
        Some(x) => Some(x.naive_local()),
        None => None,
    };
    next_time
}

pub fn get_task_end_time(cron_str: String, task_count: u64) -> Option<NaiveDateTime> {
    match task_count {
        0 => Some(Local::now().naive_local()),
        v => {
            let schedule = cron_clock::Schedule::from_str(&cron_str).unwrap();
            let end_time = match schedule.upcoming(Local).take(v.try_into().unwrap()).last() {
                Some(x) => Some(x.naive_local()),
                None => None,
            };
            end_time
        }
    }
}

//  任务执行完成，删除任务
pub async fn delete_job(task_id: u64) -> Result<()> {
    let t_builder = task_builder::TASK_TIMER.lock().await;
    match t_builder.remove_task(task_id) {
        Ok(_) => {
            println!("{}删除任务成功", task_id);
        }
        Err(e) => {
            println!("{:#?}", e);
        }
    };

    //  通过 `DelayTimer` 好像不能获取正在运行的任务，所以咋暂时无法在无任务时关闭 DelayTimer
    Ok(())
}
