use app_service::{service_utils::jwt::Claims, system};
use axum::{extract::Query, Json};
use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    system::{
        models::sys_dept::{DeptResp, RespTree, SysDeptAddReq, SysDeptDeleteReq, SysDeptEditReq, SysDeptSearchReq},
        prelude::SysDeptModel,
    },
    DB,
};

#[utoipa::path(
    get,
    path = "/system/dept/list",
    tag = "SysDept",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "获取部门列表", body = SysDeptModel),
    ),
    params(
        ("page_params" = PageParams, Query, description = "分页参数"),
        ("params" = SysDeptSearchReq, Query, description = "查询参数"),
    ),
)]
/// 获取部门列表
pub async fn get_sort_list(Query(page_params): Query<PageParams>, Query(req): Query<SysDeptSearchReq>) -> Res<ListData<SysDeptModel>> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_dept::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    post,
    path = "/system/dept/add",
    tag = "SysDept",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "新增部门", body = String)
    ),
    request_body = SysDeptAddReq,
)]
/// 新增部门
pub async fn add(Json(req): Json<SysDeptAddReq>, user: Claims) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_dept::add(db, req, user.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    delete,
    path = "/system/dept/delete",
    tag = "SysDept",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "删除部门", body = String)
    ),
    request_body = SysDeptDeleteReq,
)]
/// 删除部门
pub async fn delete(Json(req): Json<SysDeptDeleteReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_dept::delete(db, req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    put,
    path = "/system/dept/edit",
    tag = "SysDept",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "编辑部门", body = String)
    ),
    request_body = SysDeptEditReq,
)]
/// 编辑部门
pub async fn edit(Json(req): Json<SysDeptEditReq>, user: Claims) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_dept::edit(db, req, user.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    get,
    path = "/system/dept/get_by_id",
    tag = "SysDept",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "按id获取部门", body = DeptResp)
    ),
    params(
        ("params" = SysDeptSearchReq, Query, description = "查询参数")
    ),
)]
/// 按id获取部门
pub async fn get_by_id(Query(req): Query<SysDeptSearchReq>) -> Res<DeptResp> {
    let db = DB.get_or_init(db_conn).await;
    if let Some(x) = req.dept_id {
        let res = system::sys_dept::get_by_id(db, &x).await;
        match res {
            Ok(x) => Res::with_data(x),
            Err(e) => Res::with_err(&e.to_string()),
        }
    } else {
        Res::with_err("参数错误")
    }
}

#[utoipa::path(
    get,
    path = "/system/dept/get_all",
    tag = "SysDept",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "获取全部部门", body = [DeptResp])
    ),
)]
/// 获取全部部门
pub async fn get_all() -> Res<Vec<DeptResp>> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_dept::get_all(db).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    get,
    path = "/system/dept/get_dept_tree",
    tag = "SysDept",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "获取全部部门树", body = [DeptResp])
    ),
)]
/// 获取全部部门树
pub async fn get_dept_tree() -> Res<Vec<RespTree>> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_dept::get_dept_tree(db).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
