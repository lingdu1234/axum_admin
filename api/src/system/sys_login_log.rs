use app_service::system;
use axum::{extract::Query, Json};
use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    system::{
        models::sys_login_log::{SysLoginLogDeleteReq, SysLoginLogSearchReq},
        prelude::SysLoginLogModel,
    },
    DB,
};

#[utoipa::path(
    get,
    path = "/system/login-log/list",
    tag = "SysLoginLog",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "获取登录日志", body = SysLoginLogModel),
    ),
    params(
        ("page_params" = PageParams, Query, description = "分页参数"),
        ("params" = SysLoginLogSearchReq, Query, description = "查询参数"),
    ),
)]
/// 获取登录日志
pub async fn get_sort_list(Query(page_params): Query<PageParams>, Query(req): Query<SysLoginLogSearchReq>) -> Res<ListData<SysLoginLogModel>> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_login_log::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    delete,
    path = "/system/login-log/delete",
    tag = "SysLoginLog",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "删除登录日志", body = String),
    ),
    request_body = SysLoginLogDeleteReq,
)]
/// 删除登录日志
pub async fn delete(Json(delete_req): Json<SysLoginLogDeleteReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_login_log::delete(db, delete_req).await;
    println!("{:?}", res);
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    delete,
    path = "/system/login-log/clean",
    tag = "SysLoginLog",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "清空登录日志", body = String),
    ),
)]
/// 清空登录日志
pub async fn clean() -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_login_log::clean(db).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
