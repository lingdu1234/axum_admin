use axum::{extract::Query, Json};
use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    system::{
        models::sys_job_log::{SysJobLogCleanReq, SysJobLogDeleteReq, SysJobLogSearchReq},
        prelude::SysJobLogModel,
    },
    DB,
};

use super::super::service;

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0

pub async fn get_sort_list(Query(page_params): Query<PageParams>, Query(req): Query<SysJobLogSearchReq>) -> Res<ListData<SysJobLogModel>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_job_log::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// delete 完全删除

pub async fn delete(Json(req): Json<SysJobLogDeleteReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_job_log::delete(db, req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

pub async fn clean(Json(req): Json<SysJobLogCleanReq>) -> Res<String> {
    //  数据验证
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_job_log::clean(db, req.job_id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
