use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use db::{
    common::res::{ListData, PageParams},
    system::{
        entities::{prelude::SysOperLog, sys_oper_log},
        models::sys_oper_log::{DeleteReq, SearchReq},
    },
};
use sea_orm::{sea_query::Table, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};

/// get_list 获取列表
/// page_params 分页参数
pub async fn get_sort_list(db: &DatabaseConnection, page_params: PageParams, req: SearchReq) -> Result<ListData<sys_oper_log::Model>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(10);
    //  生成查询条件
    let mut s = SysOperLog::find();
    if let Some(x) = req.title {
        if !x.is_empty() {
            s = s.filter(sys_oper_log::Column::Title.eq(x));
        }
    }
    if let Some(x) = req.oper_name {
        if !x.is_empty() {
            s = s.filter(sys_oper_log::Column::OperName.contains(&x));
        }
    }

    if let Some(x) = req.operator_type {
        if !x.is_empty() {
            s = s.filter(sys_oper_log::Column::OperatorType.eq(x));
        }
    }

    if let Some(x) = req.status {
        if !x.is_empty() {
            s = s.filter(sys_oper_log::Column::Status.eq(x));
        }
    }
    if let Some(x) = req.begin_time {
        if !x.is_empty() {
            let x = x + " 00:00:00";
            let t = NaiveDateTime::parse_from_str(&x, "%Y-%m-%d %H:%M:%S")?;
            s = s.filter(sys_oper_log::Column::OperTime.gte(t));
        }
    }
    if let Some(x) = req.end_time {
        if !x.is_empty() {
            let x = x + " 23:59:59";
            let t = NaiveDateTime::parse_from_str(&x, "%Y-%m-%d %H:%M:%S")?;
            s = s.filter(sys_oper_log::Column::OperTime.lte(t));
        }
    }
    // 获取全部数据条数
    let total = s.clone().count(db).await?;
    // 分页获取数据
    let paginator = s.order_by_desc(sys_oper_log::Column::OperTime).paginate(db, page_per_size);
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

/// delete 完全删除
pub async fn delete(db: &DatabaseConnection, delete_req: DeleteReq) -> Result<String> {
    let mut s = SysOperLog::delete_many();

    s = s.filter(sys_oper_log::Column::OperId.is_in(delete_req.oper_log_ids));

    // 开始删除
    let d = s.exec(db).await.map_err(|e| anyhow!(e.to_string()))?;

    match d.rows_affected {
        0 => Err(anyhow!("你要删除的日志不存在".to_string(),)),

        i => Ok(format!("成功删除{}条数据", i)),
    }
}

/// delete 完全删除
pub async fn clean(db: &DatabaseConnection) -> Result<String> {
    let stmt = Table::truncate().table(sys_oper_log::Entity).to_owned();
    let db_backend = db.get_database_backend();
    db.execute(db_backend.build(&stmt)).await?;
    Ok("日志清空成功".to_string())
}

/// get_user_by_id 获取用户Id获取用户
/// db 数据库连接 使用db.0
pub async fn get_by_id(db: &DatabaseConnection, oper_id: String) -> Result<sys_oper_log::Model> {
    let s = SysOperLog::find().filter(sys_oper_log::Column::OperId.eq(oper_id)).one(db).await?;

    let res = match s {
        Some(m) => m,
        None => return Err(anyhow!("没有找到数据")),
    };
    Ok(res)
}
