use app_service::{service_utils::jwt::Claims, system};
use axum::{extract::Query, Json};
use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    system::{
        models::sys_post::{SysPostAddReq, SysPostDeleteReq, SysPostEditReq, SysPostResp, SysPostSearchReq},
        prelude::SysPostModel,
    },
    DB,
};

#[utoipa::path(
    get,
    path = "/system/post/list",
    tag = "SysPost",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "获取岗位列表", body = SysPostModel),
    ),
    params(
        ("page_params" = PageParams, Query, description = "分页参数"),
        ("params" = SysPostSearchReq, Query, description = "查询参数"),
    ),
)]
/// 获取岗位列表
pub async fn get_sort_list(Query(page_params): Query<PageParams>, Query(req): Query<SysPostSearchReq>) -> Res<ListData<SysPostModel>> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_post::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    post,
    path = "/system/post/add",
    tag = "SysPost",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "新增岗位", body = String)
    ),
    request_body = SysPostAddReq,
)]
/// 新增岗位
pub async fn add(Json(req): Json<SysPostAddReq>, user: Claims) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_post::add(db, req, user.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    delete,
    path = "/system/post/delete",
    tag = "SysPost",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "删除岗位", body = String)
    ),
    request_body = SysPostDeleteReq,
)]
/// 删除岗位
pub async fn delete(Json(req): Json<SysPostDeleteReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_post::delete(db, req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    put,
    path = "/system/post/edit",
    tag = "SysPost",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "修改岗位", body = String)
    ),
    request_body = SysPostEditReq,
)]
/// 修改岗位
pub async fn edit(Json(req): Json<SysPostEditReq>, user: Claims) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_post::edit(db, req, user.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    get,
    path = "/system/post/get_by_id",
    tag = "SysPost",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "按id获取岗位", body = SysPostResp)
    ),
    params(
        ("params" = SysPostSearchReq, Query, description = "查询参数")
    ),
)]
/// 按id获取岗位
pub async fn get_by_id(Query(req): Query<SysPostSearchReq>) -> Res<SysPostResp> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_post::get_by_id(db, req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    get,
    path = "/system/post/get_all",
    tag = "SysPost",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "获取全部岗位", body = SysPostResp)
    )
)]
/// 获取全部岗位
pub async fn get_all() -> Res<Vec<SysPostResp>> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_post::get_all(db).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
