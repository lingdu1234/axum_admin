use app_service::{service_utils::jwt::Claims, system};
use axum::{extract::Query, Json};
use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    system::{
        models::sys_dict_data::{SysDictDataAddReq, SysDictDataDeleteReq, SysDictDataEditReq, SysDictDataSearchReq},
        prelude::SysDictDataModel,
    },
    DB,
};

#[utoipa::path(
    get,
    path = "/system/dict/data/list",
    tag = "SysDictData",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "获取字典数据列表", body = SysDictDataModel),
    ),
    params(
        ("page_params" = PageParams, Query, description = "分页参数"),
        ("params" = SysDictDataSearchReq, Query, description = "查询参数"),
    ),
)]
/// 获取字典数据列表
pub async fn get_sort_list(Query(page_params): Query<PageParams>, Query(req): Query<SysDictDataSearchReq>) -> Res<ListData<SysDictDataModel>> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_dict_data::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    post,
    path = "/system/dict/data/add",
    tag = "SysDictData",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "新增字典数据", body = String)
    ),
    request_body = SysDictDataAddReq,
)]
/// 新增字典数据
pub async fn add(Json(req): Json<SysDictDataAddReq>, user: Claims) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_dict_data::add(db, req, user.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    delete,
    path = "/system/dict/data/delete",
    tag = "SysDictData",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "删除字典数据", body = String)
    ),
    request_body = SysDictDataDeleteReq,
)]
/// 删除字典数据
pub async fn delete(Json(req): Json<SysDictDataDeleteReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_dict_data::delete(db, req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    put,
    path = "/system/dict/data/edit",
    tag = "SysDictData",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "编辑字典数据", body = String)
    ),
    request_body = SysDictDataEditReq,
)]
/// 编辑字典数据
pub async fn edit(Json(req): Json<SysDictDataEditReq>, user: Claims) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_dict_data::edit(db, req, user.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    get,
    path = "/system/dict/data/get_by_id",
    tag = "SysDictData",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "按id获取字典数据", body = SysDictDataModel)
    ),
    params(
        ("params" = SysDeptSearchReq, Query, description = "查询参数")
    ),
)]
/// 按id获取字典数据
pub async fn get_by_id(Query(req): Query<SysDictDataSearchReq>) -> Res<SysDictDataModel> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_dict_data::get_by_id(db, req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    get,
    path = "/system/dict/data/get_by_type",
    tag = "SysDictData",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "按id获取字典数据", body = [SysDictDataModel])
    ),
    params(
        ("params" = SysDeptSearchReq, Query, description = "查询参数")
    ),
)]
/// 按type获取字典数据
pub async fn get_by_type(Query(req): Query<SysDictDataSearchReq>) -> Res<Vec<SysDictDataModel>> {
    let db = DB.get_or_init(db_conn).await;
    match system::sys_dict_data::get_by_type(db, req).await {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

// #[utoipa::path(
//     get,
//     path = "/system/dict/data/get_all",
//     tag = "SysDictData",
//     security(("authorization" = [])),
//     responses(
//         (status = 200, description = "按id获取字典数据", body =
// [SysDictDataModel])     ),
//     params(
//         ("params" = SysDeptSearchReq, Query, description = "查询参数")
//     ),
// )]
// 按获取全部字典数据
// pub async fn get_all() -> impl IntoResponse {
//     let db = DB.get_or_init(db_conn).await;
//     let res = system::sys_dict_data::get_all(db).await;
//     match res {
//         Ok(x) => Res::with_data(x),
//         Err(e) => Res::with_err(&e.to_string()),
//     }
// }
