use std::str::FromStr;

use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDateTime};
use db::{
    db_conn,
    system::{SysJobColumn, SysJobEntity, SysJobLogAddReq, SysJobModel},
    DB,
};
use delay_timer::prelude::cron_clock;
use sea_orm::{sea_query::Expr, ColumnTrait, EntityTrait, QueryFilter, TransactionTrait};
use tracing::info;

use super::{
    super::system::{get_job_by_id, sys_job_log_add},
    task, task_builder, TaskModel, TASK_MODELS,
};

pub async fn run_once_task(job_id: String, task_id: i64, is_once: bool) {
    let begin_time = Local::now().naive_local();
    let job = match is_once {
        true => {
            let db = DB.get_or_init(db_conn).await;
            let jm = get_job_by_id(db, job_id.clone()).await.expect("job not found");
            TaskModel {
                run_lot: 0_i64,
                count: 0_i64,
                lot_count: 0_i64,
                next_run_time: begin_time,
                lot_end_time: begin_time,
                model: jm,
            }
        }
        false => {
            let task_models = TASK_MODELS.lock().await;
            let res = task_models.get(&task_id).cloned().expect("task not found");
            drop(task_models);
            res
        }
    };
    let cron_str = job.model.cron_expression.clone();
    let next_time = match get_next_task_run_time(cron_str) {
        Ok(x) => match x {
            Some(y) => y,
            None => return,
        },
        Err(_) => {
            info!("cron 表格式解析错误");
            return;
        }
    };
    let res = task::go_run_task(job.model.job_params.clone(), job.model.invoke_target.clone()).await;
    let task_end_time = Local::now().naive_local(); // 获取结束时间
    let elapsed_time: i64 = task_end_time.signed_duration_since(begin_time).to_std().unwrap().as_millis().try_into().unwrap_or(i64::MAX); // 获取结束时间和开始时间的时间差，单位为毫秒
    tokio::spawn(async move {
        match is_once {
            true => write_once_job_log(res, job, begin_time, elapsed_time).await,
            false => {
                let mut task_models = TASK_MODELS.lock().await;
                task_models.entry(task_id).and_modify(|x| {
                    x.lot_count += 1;
                    x.next_run_time = next_time;
                });
                let job_new = task_models.get(&task_id).cloned().expect("task not found");
                write_circle_job_log(res, job_new, begin_time, next_time, elapsed_time).await;
                drop(task_models);
            }
        }
    });
}

pub async fn add_circles_task(t: SysJobModel) -> Result<()> {
    let task_count = t.task_count;
    let task = task_builder::build_task(&t.job_id, &t.cron_expression, &t.job_name, task_count as u64, t.task_id.try_into().unwrap_or(0));
    match task {
        Ok(x) => {
            init_task_model(t, task_count).await;
            let t_builder = task_builder::TASK_TIMER.write().await;
            match t_builder.add_task(x) {
                Ok(_) => {
                    drop(t_builder);
                }
                Err(e) => {
                    drop(t_builder);
                    return Err(anyhow!("{:#?}", e));
                }
            };
        }
        Err(e) => return Err(anyhow!("{:#?}", e)),
    };
    Ok(())
}

pub async fn update_circles_task(t: SysJobModel) -> Result<()> {
    let task_count = match t.task_count {
        x @ 0..=9999 => x, // 防止程序卡死,更新任务时，限制最大任务数
        _ => 9999_i64,
    };
    let task = task_builder::build_task(&t.job_id, &t.cron_expression, &t.job_name, task_count as u64, t.task_id.try_into().unwrap_or(0));
    let remark_update_info_t = format!(
        "任务更新:--------    更新时间:{}\n任务名称:{}修改后次数:{}\n任务时间:{}\n调用方法:{}\n调用方法:{}",
        Local::now().naive_local().format("%Y-%m-%d %H:%M:%S"),
        &t.job_name,
        task_count,
        &t.cron_expression,
        &t.invoke_target,
        &t.job_params.clone().unwrap_or_default()
    );
    tracing::info!("定时任务更新:{}", &remark_update_info_t);
    match task {
        Ok(x) => {
            let t_builder = task_builder::TASK_TIMER.write().await;
            match t_builder.update_task(x) {
                Ok(_) => {
                    drop(t_builder);
                    let mut task_models = TASK_MODELS.lock().await;
                    let mut remark = t.remark.clone().unwrap_or_default() + &remark_update_info_t;
                    task_models.entry(t.task_id).and_modify(|x| {
                        x.model = t.clone();
                        remark = remark.clone() + "    已运行次数:" + x.lot_count.to_string().as_str() + "\n";
                        x.model.remark = Some(remark.clone());
                        x.count = task_count;
                        x.next_run_time = match get_next_task_run_time(t.cron_expression.clone()) {
                            Ok(x) => match x {
                                Some(y) => y,
                                None => Local::now().naive_local(),
                            },
                            Err(_) => Local::now().naive_local(),
                        };
                        x.lot_end_time = get_task_end_time(t.cron_expression.clone(), task_count as u64).unwrap();
                    });
                    drop(task_models);
                    tokio::spawn(async move {
                        let db = DB.get_or_init(db_conn).await;
                        SysJobEntity::update_many()
                            .col_expr(SysJobColumn::Remark, Expr::value(remark.clone()))
                            .col_expr(SysJobColumn::TaskCount, Expr::value(task_count))
                            .filter(SysJobColumn::JobId.eq(t.job_id.clone()))
                            .exec(db)
                            .await
                            .expect("update job log failed");
                    });
                }
                Err(e) => {
                    drop(t_builder);
                    return Err(anyhow!("{:#?}", e));
                }
            };
        }
        Err(e) => return Err(anyhow!("{:#?}", e)),
    };
    Ok(())
}

async fn init_task_model(m: SysJobModel, task_count: i64) {
    let now = Local::now().naive_local();
    let run_lot = now.timestamp();
    let job_end_time = get_task_end_time(m.cron_expression.clone(), m.task_count.try_into().unwrap()).unwrap();
    let next_time = match get_next_task_run_time(m.cron_expression.clone()) {
        Ok(v) => match v {
            Some(x) => x,
            None => Local::now().naive_local(),
        },
        Err(_) => Local::now().naive_local(),
    };
    let mut task_models = TASK_MODELS.lock().await;
    let mut task_model = TaskModel {
        run_lot,
        count: task_count,
        lot_count: 0,
        next_run_time: next_time,
        lot_end_time: job_end_time,
        model: m.clone(),
    };
    let remark = format!("任务id:{}    开始时间:{}\n", run_lot, now.format("%Y-%m-%d %H:%M:%S"),);
    task_model.model.remark = Some(remark.clone());
    task_models.insert(m.task_id, task_model);
    drop(task_models);
    tokio::spawn(async move {
        let db = DB.get_or_init(db_conn).await;
        SysJobEntity::update_many()
            .col_expr(SysJobColumn::NextTime, Expr::value(next_time))
            .col_expr(SysJobColumn::RunCount, Expr::value(0_i64))
            // .col_expr(SysJobColumn::TaskCount, Expr::value(task_count))
            .col_expr(SysJobColumn::Remark, Expr::value(remark.clone()))
            .filter(SysJobColumn::JobId.eq(m.job_id.clone()))
            .exec(db)
            .await
            .expect("update job log failed");
    });
}

async fn write_circle_job_log(res: Result<String>, job: TaskModel, begin_time: NaiveDateTime, next_time: NaiveDateTime, elapsed_time: i64) {
    let (job_message, exception_info, status) = match res {
        Ok(x) => (Some(x), None, "1".to_string()),
        Err(e) => (None, Some(format!("{:#?}", e)), "0".to_string()),
    };
    let job_remark_t = job.model.remark.clone().unwrap_or_default();
    // 获取结束时间和开始时间的时间差，单位为毫秒

    let (job_remark, job_status) = match job.count != 0 && job.count <= job.lot_count {
        false => (job_remark_t, "1".to_string()),
        true => {
            delete_job(job.model.task_id, false).await.expect("delete job failed");
            let job_remark = job_remark_t + format!("任务完毕:--------    完成时间:{}\n最终运行次数:{}", begin_time, job.lot_count).as_str();
            (job_remark, "0".to_string())
        }
    };
    let job_log = SysJobLogAddReq {
        job_id: job.model.job_id.clone(),
        lot_id: job.run_lot,
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
    sys_job_log_add(&txn, job_log).await.expect("write job log failed");

    SysJobEntity::update_many()
        .col_expr(SysJobColumn::Status, Expr::value(job_status))
        .col_expr(SysJobColumn::Remark, Expr::value(job_remark))
        .col_expr(SysJobColumn::LastTime, Expr::value(begin_time))
        .col_expr(SysJobColumn::NextTime, Expr::value(next_time))
        .col_expr(SysJobColumn::RunCount, Expr::value(job.lot_count))
        .filter(SysJobColumn::JobId.eq(job.model.job_id.clone()))
        .exec(&txn)
        .await
        .expect("update job log failed");
    txn.commit().await.expect("事务提交失败");
}

async fn write_once_job_log(res: Result<String>, job: TaskModel, begin_time: NaiveDateTime, elapsed_time: i64) {
    let (job_message, exception_info, status) = match res {
        Ok(x) => (Some(x), None, "1".to_string()),
        Err(e) => (None, Some(format!("{:#?}", e)), "0".to_string()),
    };

    let job_log = SysJobLogAddReq {
        job_id: job.model.job_id.clone(),
        lot_id: 0_i64,
        lot_order: 0_i64,
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
    sys_job_log_add(&txn, job_log).await.expect("write job log failed");
    txn.commit().await.expect("事务提交失败");
}

pub fn get_next_task_run_time(cron_str: String) -> Result<Option<NaiveDateTime>> {
    let schedule = cron_clock::Schedule::from_str(&cron_str)?;
    let next_time = schedule.upcoming(Local).next().map(|x| x.naive_local());
    Ok(next_time)
}

pub fn get_task_end_time(cron_str: String, task_count: u64) -> Option<NaiveDateTime> {
    match task_count {
        0 => Some(Local::now().naive_local()),
        v => {
            let schedule = cron_clock::Schedule::from_str(&cron_str).unwrap();
            let end_time = schedule.upcoming(Local).take(v.try_into().unwrap()).last().map(|x| x.naive_local());
            end_time
        }
    }
}

//  任务执行完成，删除任务
pub async fn delete_job(task_id: i64, is_manual: bool) -> Result<()> {
    let t_builder = task_builder::TASK_TIMER.write().await;
    match t_builder.remove_task(task_id as u64) {
        Ok(_) => match is_manual {
            false => {
                drop(t_builder);
                return Ok(());
            }
            true => {
                drop(t_builder);
                let task_models = TASK_MODELS.lock().await;
                let job = match task_models.get(&task_id).cloned() {
                    Some(v) => v,
                    None => return Ok(()), // 任务不存在直接返回，数据库删除任务
                };
                drop(task_models);
                let db = DB.get_or_init(db_conn).await;
                let remark = job.clone().model.remark.unwrap_or_default()
                    + format!("任务删除:--------    删除时间:{}\n最终运行次数:{}", Local::now().naive_local(), job.lot_count,).as_str();
                SysJobEntity::update_many()
                    .col_expr(SysJobColumn::Status, Expr::value("0".to_string()))
                    .col_expr(SysJobColumn::Remark, Expr::value(remark))
                    .filter(SysJobColumn::JobId.eq(job.model.job_id.clone()))
                    .exec(db)
                    .await
                    .expect("update job log failed");
            }
        },
        Err(e) => {
            drop(t_builder);
            return Err(anyhow!("delete task failed, {}", e.to_string()));
        }
    };

    //  通过 `DelayTimer` 好像不能获取正在运行的任务，所以咋暂时无法在无任务时关闭
    // DelayTimer
    Ok(())
}
