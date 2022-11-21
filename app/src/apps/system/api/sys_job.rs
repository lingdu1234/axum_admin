use axum::{extract::Query, Json};
use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    system::{
        models::sys_job::{JobId, SysJobAddReq, SysJobDeleteReq, SysJobEditReq, SysJobSearchReq, SysJobStatusReq, ValidateReq, ValidateRes},
        SysJobModel,
    },
    DB,
};

use super::super::service;
use crate::{tasks, utils::jwt::Claims};

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0

pub async fn get_sort_list(Query(page_params): Query<PageParams>, Query(req): Query<SysJobSearchReq>) -> Res<ListData<SysJobModel>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_job::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
/// add 添加

pub async fn add(Json(req): Json<SysJobAddReq>, user: Claims) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_job::add(db, req, user.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// delete 完全删除

pub async fn delete(Json(req): Json<SysJobDeleteReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_job::delete(db, req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

// edit 修改

pub async fn edit(Json(edit_req): Json<SysJobEditReq>, user: Claims) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_job::edit(db, edit_req, user.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// get_user_by_id 获取用户Id获取用户
/// db 数据库连接 使用db.0

pub async fn get_by_id(Query(req): Query<SysJobSearchReq>) -> Res<SysJobModel> {
    let id = match req.job_id {
        None => return Res::with_err("id不能为空"),
        Some(x) => x,
    };
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_job::get_by_id(db, id).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

pub async fn change_status(Json(req): Json<SysJobStatusReq>) -> Res<String> {
    //  数据验证
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_job::set_status(db, req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

pub async fn run_task_once(Json(req): Json<JobId>) -> Res<String> {
    tasks::run_once_task(req.job_id, req.task_id, true).await;
    Res::with_msg("任务开始执行")
}

pub async fn validate_cron_str(Json(req): Json<ValidateReq>) -> Res<ValidateRes> {
    let res = service::sys_job::validate_cron_str(req.cron_str);
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
