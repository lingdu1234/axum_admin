use app_service::{service_utils::jwt::Claims, system};
use axum::{extract::Query, response::IntoResponse, Json};
use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    system::{
        models::sys_dict_type::{SysDictTypeAddReq, SysDictTypeDeleteReq, SysDictTypeEditReq, SysDictTypeSearchReq},
        prelude::SysDictTypeModel,
    },
    DB,
};

#[utoipa::path(
    get,
    path = "/system/dict/type/list",
    tag = "SysDictType",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "获取字典数据列表", body = SysDictTypeModel),
    ),
    params(
        ("page_params" = PageParams, Query, description = "分页参数"),
        ("params" = SysDictTypeSearchReq, Query, description = "查询参数"),
    ),
)]
/// 获取字典数据列表
pub async fn get_sort_list(Query(page_params): Query<PageParams>, Query(req): Query<SysDictTypeSearchReq>) -> Res<ListData<SysDictTypeModel>> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_dict_type::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    post,
    path = "/system/dict/type/add",
    tag = "SysDictType",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "新增字典数据", body = String)
    ),
    request_body = SysDictTypeAddReq,
)]
/// 新增字典数据
pub async fn add(Json(req): Json<SysDictTypeAddReq>, user: Claims) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_dict_type::add(db, req, user.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    delete,
    path = "/system/dict/type/delete",
    tag = "SysDictType",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "删除字典数据", body = String)
    ),
    request_body = SysDictTypeDeleteReq,
)]
/// 删除字典数据
pub async fn delete(Json(req): Json<SysDictTypeDeleteReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_dict_type::delete(db, req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    put,
    path = "/system/dict/type/edit",
    tag = "SysDictType",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "编辑字典数据", body = String)
    ),
    request_body = SysDictTypeEditReq,
)]
/// 编辑字典数据
pub async fn edit(Json(edit_req): Json<SysDictTypeEditReq>, user: Claims) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_dict_type::edit(db, edit_req, user.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    get,
    path = "/system/dict/type/get_by_id",
    tag = "SysDictType",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "按id获取字典数据", body = SysDictTypeModel)
    ),
    params(
        ("params" = SysDeptSearchReq, Query, description = "查询参数")
    ),
)]
/// 按id获取字典数据
pub async fn get_by_id(Query(req): Query<SysDictTypeSearchReq>) -> impl IntoResponse {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_dict_type::get_by_id(db, req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    get,
    path = "/system/dict/type/get_all",
    tag = "SysDictType",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "按id获取字典数据", body = [SysDictTypeModel])
    )
)]
/// 按id获取字典数据
pub async fn get_all() -> impl IntoResponse {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_dict_type::get_all(db).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
