use app_service::{service_utils::jwt::Claims, system};
use axum::Json;
use db::{
    common::res::Res,
    db_conn,
    system::{
        models::sys_update_log::{SysUpdateLogAddReq, SysUpdateLogDeleteReq, SysUpdateLogEditReq},
        prelude::SysUpdateLogModel,
    },
    DB,
};

#[utoipa::path(
    post,
    path = "/system/update_log/add",
    tag = "SysUpdateLog",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "新增更新日志", body = String)
    ),
    request_body = SysUpdateLogAddReq,
)]
/// 新增更新日志
pub async fn add(Json(req): Json<SysUpdateLogAddReq>, user: Claims) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_update_log::add(db, req, &user.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    delete,
    path = "/system/update_log/delete",
    tag = "SysUpdateLog",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "删除更新日志", body = String)
    ),
    request_body = SysUpdateLogDeleteReq,
)]
/// 删除更新日志
pub async fn delete(Json(req): Json<SysUpdateLogDeleteReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_update_log::soft_delete(db, &req.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    put,
    path = "/system/update_log/edit",
    tag = "SysUpdateLog",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "删除更新日志", body = String)
    ),
    request_body = SysUpdateLogEditReq,
)]
/// 删除更新日志
pub async fn edit(Json(req): Json<SysUpdateLogEditReq>, user: Claims) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_update_log::edit(db, req, &user.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    get,
    path = "/system/update_log/get_all",
    tag = "SysUpdateLog",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "获取全部更新日志", body = [SysUpdateLogModel])
    )
)]
/// 获取全部更新日志
pub async fn get_all() -> Res<Vec<SysUpdateLogModel>> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_update_log::get_all(db).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
