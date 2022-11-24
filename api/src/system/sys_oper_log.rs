use axum::{extract::Query, Json};
use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    system::{
        models::sys_oper_log::{SysOperLogDeleteReq, SysOperLogSearchReq},
        prelude::SysOperLogModel,
    },
    DB,
};

use super::super::service;


#[utoipa::path(
    get,
    path = "/system/oper_log/list",
    tag = "SysOperLog",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "获取操作日志", body = SysOperLogModel),
    ),
    params(
        ("page_params" = PageParams, Query, description = "分页参数"),
        ("params" = SysOperLogSearchReq, Query, description = "查询参数"),
    ),
)]
/// 获取操作日志
pub async fn get_sort_list(Query(page_params): Query<PageParams>, Query(req): Query<SysOperLogSearchReq>) -> Res<ListData<SysOperLogModel>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_oper_log::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    delete,
    path = "/system/oper_log/delete",
    tag = "SysOperLog",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "删除操作日志", body = String),
    ),
    request_body = SysOperLogDeleteReq,
)]
/// 删除操作日志
pub async fn delete(Json(req): Json<SysOperLogDeleteReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_oper_log::delete(db, req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    delete,
    path = "/system/oper_log/clean",
    tag = "SysOperLog",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "删除操作日志", body = String),
    ),
)]
/// 删除操作日志
pub async fn clean() -> Res<String> {
    //  数据验证
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_oper_log::clean(db).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}


#[utoipa::path(
    get,
    path = "/system/oper_log/get_by_id",
    tag = "SysOperLog",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "按ID获取登录日志", body = SysOperLogModel),
    ),
    params(
        ("params" = SysOperLogSearchReq, Query, description = "查询参数"),
    ),
)]
/// 按ID获取登录日志
pub async fn get_by_id(Query(req): Query<SysOperLogSearchReq>) -> Res<SysOperLogModel> {
    let id = match req.oper_id {
        None => return Res::with_err("id不能为空"),
        Some(x) => x,
    };
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_oper_log::get_by_id(db, id).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
