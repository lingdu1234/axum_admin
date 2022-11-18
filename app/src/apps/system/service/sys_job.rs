use std::str::FromStr;

use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDateTime};
use db::{
    common::res::{ListData, PageParams},
    system::{
        entities::{prelude::SysJob, sys_job},
        models::sys_job::{AddReq, DeleteReq, EditReq, SearchReq, StatusReq, ValidateRes},
    },
};
use delay_timer::prelude::cron_clock;
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder, Set,
    TransactionTrait,
};

use crate::tasks;

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
pub async fn get_sort_list(db: &DatabaseConnection, page_params: PageParams, search_req: SearchReq) -> Result<ListData<sys_job::Model>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(10);
    //  生成查询条件
    let mut s = SysJob::find();

    if let Some(x) = search_req.job_name {
        if !x.is_empty() {
            s = s.filter(sys_job::Column::JobName.contains(&x));
        }
    }

    if let Some(x) = search_req.job_group {
        if !x.is_empty() {
            s = s.filter(sys_job::Column::JobGroup.contains(&x));
        }
    }
    if let Some(x) = search_req.status {
        if !x.is_empty() {
            s = s.filter(sys_job::Column::Status.eq(x));
        }
    }
    // 获取全部数据条数
    let total = s.clone().count(db).await?;
    // 分页获取数据
    let paginator = s.order_by_asc(sys_job::Column::JobId).paginate(db, page_per_size);
    let total_pages = paginator.num_pages().await?;
    let list = paginator.fetch_page(page_num - 1).await?;

    let res = ListData {
        total,
        list,
        total_pages,
        page_num,
    };
    Ok(res)
}

pub async fn check_job_add_is_exist<C>(db: &C, job_name: &str, task_id: i64) -> Result<bool>
where
    C: TransactionTrait + ConnectionTrait,
{
    let c1 = SysJob::find().filter(sys_job::Column::JobName.eq(job_name)).count(db).await?;
    let c2 = SysJob::find().filter(sys_job::Column::TaskId.eq(task_id)).count(db).await?;
    Ok(c1 > 0 || c2 > 0)
}

pub async fn check_job_edit_is_exist<C>(db: &C, job_name: &str, task_id: i64, job_id: &str) -> Result<bool>
where
    C: TransactionTrait + ConnectionTrait,
{
    let c1 = SysJob::find()
        .filter(sys_job::Column::JobName.eq(job_name))
        .filter(sys_job::Column::JobId.ne(job_id))
        .count(db)
        .await?;
    let c2 = SysJob::find()
        .filter(sys_job::Column::TaskId.eq(task_id))
        .filter(sys_job::Column::JobId.ne(job_id))
        .count(db)
        .await?;
    Ok(c1 > 0 || c2 > 0)
}

/// add 添加
pub async fn add<C>(db: &C, req: AddReq, user_id: String) -> Result<String>
where
    C: TransactionTrait + ConnectionTrait,
{
    //  检查字典类型是否存在
    if check_job_add_is_exist(db, &req.job_name, req.task_id).await? {
        return Err(anyhow!("任务已存在"));
    }
    let uid = scru128::new_string();
    let now: NaiveDateTime = Local::now().naive_local();
    let next_time = match tasks::get_next_task_run_time(req.cron_expression.to_string()) {
        Ok(v) => v,
        Err(_) => return Err(anyhow!("cron 表达式解析错误")),
    };
    let status = req.status.unwrap_or_else(|| "1".to_string());
    let add_data = sys_job::ActiveModel {
        job_id: Set(uid.clone()),
        task_id: Set(req.task_id),
        task_count: Set(req.task_count),
        run_count: Set(0),
        job_name: Set(req.job_name),
        job_params: Set(req.job_params),
        job_group: Set(req.job_group),
        invoke_target: Set(req.invoke_target),
        cron_expression: Set(req.cron_expression),
        misfire_policy: Set(req.misfire_policy),
        concurrent: Set(req.concurrent),
        status: Set(status.clone()),
        remark: Set(req.remark),
        next_time: Set(next_time),
        create_by: Set(user_id),
        created_at: Set(Some(now)),
        ..Default::default()
    };
    SysJob::insert(add_data.clone()).exec(db).await?;
    if status.as_str() == "1" {
        tasks::run_circles_task(uid.clone()).await.expect("任务添加失败");
    };

    let res = format!("{}添加成功", uid);

    Ok(res)
}

/// delete 完全删除
pub async fn delete(db: &DatabaseConnection, delete_req: DeleteReq) -> Result<String> {
    let job_ids = delete_req.job_ids.clone();
    // 删除任务
    for job_id in job_ids.clone() {
        if let Some(m) = SysJob::find().filter(sys_job::Column::JobId.eq(job_id)).one(db).await? {
            tasks::delete_job(m.task_id, true).await.expect("任务删除失败");
        };
    }

    let d = SysJob::delete_many()
        .filter(sys_job::Column::JobId.is_in(job_ids.clone()))
        .exec(db)
        .await
        .map_err(|e| anyhow!(e.to_string(),))?;

    match d.rows_affected {
        0 => Err(anyhow!("你要删除的任务不存在",)),

        i => Ok(format!("成功删除{}条数据", i)),
    }
}

// edit 修改
pub async fn edit(db: &DatabaseConnection, req: EditReq, user_id: String) -> Result<String> {
    //  检查字典类型是否存在
    if check_job_edit_is_exist(db, &req.job_name, req.task_id, &req.job_id).await? {
        return Err(anyhow!("任务已存在",));
    }
    let uid = req.job_id;
    let s_s = get_by_id(db, uid.clone()).await?;
    let s_r: sys_job::ActiveModel = s_s.clone().into();
    let next_time = match tasks::get_next_task_run_time(req.cron_expression.to_string()) {
        Ok(v) => v,
        Err(_) => return Err(anyhow!("cron 表达式解析错误")),
    };
    let status = req.status.unwrap_or_else(|| "1".to_string());
    let now: NaiveDateTime = Local::now().naive_local();
    let act = sys_job::ActiveModel {
        job_id: Set(uid.clone()),
        task_id: Set(req.task_id),
        task_count: Set(req.task_count),
        job_name: Set(req.job_name),
        job_params: if let Some(x) = req.job_params { Set(Some(x)) } else { NotSet },
        job_group: Set(req.job_group),
        invoke_target: Set(req.invoke_target),
        cron_expression: Set(req.cron_expression),
        misfire_policy: Set(req.misfire_policy),
        concurrent: if let Some(x) = req.concurrent { Set(Some(x)) } else { NotSet },
        next_time: Set(next_time),
        status: Set(status.clone()),
        remark: Set(Some(req.remark.unwrap_or_default())),
        update_by: Set(Some(user_id)),
        updated_at: Set(Some(now)),
        ..s_r
    };
    // 更新
    act.update(db).await?;
    let job_id = uid.clone();
    tokio::spawn(async move {
        match (s_s.status.as_str(), status.clone().as_str()) {
            ("0", "1") => {
                tasks::run_circles_task(job_id.clone()).await.expect("任务执行失败");
            }
            ("1", "0") => {
                tasks::delete_job(s_s.clone().task_id, true).await.expect("任务删除失败");
            }
            ("1", "1") => {
                tracing::info!("任务状态未变化================================================");
                tasks::update_circles_task(job_id.clone()).await.expect("任务更新失败");
                // tasks::delete_job(s_s.clone().task_id,
                // true).await.expect("任务删除失败");
                // tasks::run_circles_task(uid.clone()).await.expect("
                // 任务添加失败" );
            }
            (_, _) => {
                tracing::info!("任务状态未变化+++++++++++++++++++++++++++++++++++++");
            }
        };
    });
    Ok(format!("{}修改成功", uid))
}

/// get_user_by_id 获取用户Id获取用户
/// db 数据库连接 使用db.0
pub async fn get_by_id<C>(db: &C, job_id: String) -> Result<sys_job::Model>
where
    C: TransactionTrait + ConnectionTrait,
{
    let s = SysJob::find().filter(sys_job::Column::JobId.eq(job_id)).one(db).await?;

    let res = match s {
        Some(m) => m,
        None => return Err(anyhow!("没有找到数据",)),
    };
    Ok(res)
}

/// get_all 获取全部
/// db 数据库连接 使用db.0
pub async fn get_active_job(db: &DatabaseConnection) -> Result<Vec<sys_job::Model>> {
    let s = SysJob::find()
        // .filter(sys_job::Column::DeletedAt.is_null())
        .filter(sys_job::Column::Status.eq("1".to_string()))
        .order_by(sys_job::Column::JobId, Order::Asc)
        .all(db)
        .await?;
    Ok(s)
}

/// delete 完全删除
pub async fn set_status(db: &DatabaseConnection, req: StatusReq) -> Result<String> {
    let job = get_by_id(db, req.job_id.clone()).await?;
    sys_job::Entity::update_many()
        .col_expr(sys_job::Column::Status, Expr::value(req.status.clone()))
        .filter(sys_job::Column::JobId.eq(req.job_id.clone()))
        .exec(db)
        .await?;
    match req.status.clone().as_str() {
        "1" => {
            tasks::run_circles_task(job.clone().job_id).await.expect("任务执行失败");
        }
        "0" => {
            tasks::delete_job(job.clone().task_id, true).await.expect("任务删除失败");
        }
        _ => return Err(anyhow!("状态值错误",)),
    };
    Ok(format!("{}修改成功", req.job_id))
}

/// 验证cron字符串
pub fn validate_cron_str(cron_str: String) -> Result<ValidateRes> {
    let schedule = match cron_clock::Schedule::from_str(&cron_str) {
        Ok(v) => v,
        Err(_) => return Ok(ValidateRes { validate: false, next_ten: None }),
    };
    let next_ten: Vec<NaiveDateTime> = schedule.upcoming(Local).take(12).map(|x| x.naive_local()).collect();
    Ok(ValidateRes {
        validate: true,
        next_ten: Some(next_ten),
    })
}
