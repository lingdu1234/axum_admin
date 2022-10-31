use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use db::{
    common::res::{ListData, PageParams},
    system::{
        entities::{prelude::SysJobLog, sys_job_log},
        models::sys_job_log::{AddReq, DeleteReq, SearchReq},
    },
};
use sea_orm::{sea_query::Table, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait};
/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
pub async fn get_sort_list(db: &DatabaseConnection, page_params: PageParams, req: SearchReq) -> Result<ListData<sys_job_log::Model>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(10);
    //  生成查询条件
    let mut s = SysJobLog::find();
    if let Some(x) = req.job_id {
        if !x.is_empty() {
            s = s.filter(sys_job_log::Column::JobId.eq(x));
        }
    }
    if let Some(x) = req.job_name {
        if !x.is_empty() {
            s = s.filter(sys_job_log::Column::JobName.contains(&x));
        }
    }

    if let Some(x) = req.job_group {
        if !x.is_empty() {
            s = s.filter(sys_job_log::Column::JobGroup.eq(x));
        }
    }
    if let Some(x) = req.is_once {
        if !x.is_empty() {
            s = s.filter(sys_job_log::Column::IsOnce.eq(x));
        }
    }
    if let Some(x) = req.status {
        if !x.is_empty() {
            s = s.filter(sys_job_log::Column::Status.eq(x));
        }
    }
    if let Some(x) = req.begin_time {
        let x = x + " 00:00:00";
        let t = NaiveDateTime::parse_from_str(&x, "%Y-%m-%d %H:%M:%S")?;
        s = s.filter(sys_job_log::Column::CreatedAt.gte(t));
    }
    if let Some(x) = req.end_time {
        let x = x + " 23:59:59";
        let t = NaiveDateTime::parse_from_str(&x, "%Y-%m-%d %H:%M:%S")?;
        s = s.filter(sys_job_log::Column::CreatedAt.lte(t));
    }
    // 获取全部数据条数
    let total = s.clone().count(db).await?;
    // 分页获取数据
    let paginator = s
        .order_by_desc(sys_job_log::Column::LotId)
        .order_by_desc(sys_job_log::Column::LotOrder)
        .order_by_desc(sys_job_log::Column::CreatedAt)
        .paginate(db, page_per_size);
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

/// add 添加
pub async fn add<C>(db: &C, req: AddReq) -> Result<String>
where
    C: TransactionTrait + ConnectionTrait,
{
    let uid = scru128::new_string();
    let add_data = sys_job_log::ActiveModel {
        job_log_id: Set(uid.clone()),
        job_id: Set(req.job_id),
        lot_id: Set(req.lot_id),
        lot_order: Set(req.lot_order),
        job_name: Set(req.job_name),
        job_params: Set(req.job_params),
        job_group: Set(req.job_group),
        invoke_target: Set(req.invoke_target),
        status: Set(req.status),
        created_at: Set(req.created_at),
        job_message: Set(req.job_message),
        exception_info: Set(req.exception_info),
        elapsed_time: Set(req.elapsed_time),
        is_once: Set(req.is_once),
    };
    SysJobLog::insert(add_data).exec(db).await?;

    let res = format!("{}添加成功", uid);

    Ok(res)
}

/// delete 完全删除
pub async fn delete(db: &DatabaseConnection, delete_req: DeleteReq) -> Result<String> {
    let mut s = SysJobLog::delete_many();

    s = s.filter(sys_job_log::Column::JobLogId.is_in(delete_req.job_log_ids));

    // 开始删除
    let d = s.exec(db).await.map_err(|e| anyhow!(e.to_string(),))?;

    match d.rows_affected {
        // 0 => return Err("你要删除的字典类型不存在".into()),
        0 => Err(anyhow!("你要删除的日志不存在".to_string(),)),

        i => Ok(format!("成功删除{}条数据", i)),
    }
}

/// delete 完全删除
pub async fn clean(db: &DatabaseConnection, job_id: String) -> Result<String> {
    if job_id.is_empty() {
        let stmt = Table::truncate().table(sys_job_log::Entity).to_owned();
        let db_backend = db.get_database_backend();
        db.execute(db_backend.build(&stmt)).await?;
        Ok("定时任务日志清空成功".to_string())
    } else {
        let mut s = SysJobLog::delete_many();
        s = s.filter(sys_job_log::Column::JobId.eq(job_id));
        // 开始删除
        let d = s.exec(db).await.map_err(|e| anyhow!(e.to_string(),))?;
        match d.rows_affected {
            // 0 => return Err("你要删除的字典类型不存在".into()),
            0 => Err(anyhow!("你要删除的日志不存在".to_string(),)),

            i => Ok(format!("成功删除{}条数据", i)),
        }
    }
}

/// get_user_by_id 获取用户Id获取用户
pub async fn get_by_id(db: &DatabaseConnection, job_log_id: String) -> Result<sys_job_log::Model> {
    let s = SysJobLog::find().filter(sys_job_log::Column::JobLogId.eq(job_log_id)).one(db).await?;

    let res = match s {
        Some(m) => m,
        None => return Err(anyhow!("没有找到数据",)),
    };
    Ok(res)
}
