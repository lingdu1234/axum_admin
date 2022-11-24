use app_service::{service_utils::jwt::Claims, system};
use axum::{extract::Query, Json};
use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    system::{
        models::sys_user_online::{SysUserOnlineDeleteReq, SysUserOnlineSearchReq},
        prelude::SysUserOnlineModel,
    },
    DB,
};

#[utoipa::path(
    get,
    path = "/system/online/list",
    tag = "SysUserOnline",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "获取在线用户列表", body = SysPostModel),
    ),
    params(
        ("page_params" = PageParams, Query, description = "分页参数"),
        ("params" = SysUserOnlineSearchReq, Query, description = "查询参数"),
    ),
)]
/// 获取在线用户列表
pub async fn get_sort_list(Query(page_params): Query<PageParams>, Query(req): Query<SysUserOnlineSearchReq>) -> Res<ListData<SysUserOnlineModel>> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user_online::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    delete,
    path = "/system/online/delete",
    tag = "SysUserOnline",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "强制用户下线", body = String)
    ),
    request_body = SysUserOnlineDeleteReq,
)]
/// 强制用户下线
pub async fn delete(Json(delete_req): Json<SysUserOnlineDeleteReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user_online::delete(db, delete_req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    post,
    path = "/comm/log_out",
    tag = "common",
    security(("authorization" = [])),
    responses(
        (status = 200, description = " 用户自己退出登录，下线", body = String)
    )
)]
/// 用户自己退出登录，下线
pub async fn log_out(user: Claims) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user_online::log_out(db, user.token_id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
