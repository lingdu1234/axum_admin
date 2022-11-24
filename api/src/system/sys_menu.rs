use axum::{extract::Query, Json};
use configs::CFG;
use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    system::{
        models::sys_menu::{LogCacheEditReq, MenuRelated, MenuResp, SysMenuAddReq, SysMenuDeleteReq, SysMenuEditReq, SysMenuSearchReq, SysMenuTree, SysMenuTreeAll},
        prelude::SysMenuModel,
    },
    DB,
};

use super::super::service;
use crate::utils::jwt::Claims;

#[utoipa::path(
    get,
    path = "/system/menu/list",
    tag = "SysMenu",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "获取菜单列表", body = SysMenuModel),
    ),
    params(
        ("page_params" = PageParams, Query, description = "分页参数"),
        ("params" = SysMenuSearchReq, Query, description = "查询参数"),
    ),
)]
/// 获取菜单列表 （弃用）
pub async fn get_sort_list(Query(page_params): Query<PageParams>, Query(search_req): Query<SysMenuSearchReq>) -> Res<ListData<SysMenuModel>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_menu::get_sort_list(db, page_params, search_req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    get,
    path = "/system/menu/get_by_id",
    tag = "SysMenu",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "按id获取菜单", body = MenuResp),
    ),
    params(
        ("params" = SysJobSearchReq, Query, description = "查询参数"),
    ),
)]
/// 按id获取菜单
pub async fn get_by_id(Query(search_req): Query<SysMenuSearchReq>) -> Res<MenuResp> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_menu::get_by_id(db, search_req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    post,
    path = "/system/menu/add",
    tag = "SysMenu",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "新增菜单", body = String),
    ),
    request_body = SysMenuAddReq,
)]
/// 新增菜单
pub async fn add(Json(req): Json<SysMenuAddReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_menu::add(db, req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    delete,
    path = "/system/menu/delete",
    tag = "SysMenu",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "删除菜单", body = String),
    ),
    request_body = SysMenuDeleteReq,
)]
/// 删除菜单
pub async fn delete(Json(req): Json<SysMenuDeleteReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_menu::delete(db, &req.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    put,
    path = "/system/menu/edit",
    tag = "SysMenu",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "更新菜单", body = String),
    ),
    request_body = SysMenuEditReq,
)]
/// 更新菜单
pub async fn edit(Json(edit_req): Json<SysMenuEditReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_menu::edit(db, edit_req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    put,
    path = "/system/menu/update_log_cache_method",
    tag = "SysMenu",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "更新菜单", body = String),
    ),
    request_body = LogCacheEditReq,
)]
/// 修改菜单 日志 缓存 方法
pub async fn update_log_cache_method(Json(edit_req): Json<LogCacheEditReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_menu::update_log_cache_method(db, edit_req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    get,
    path = "/system/menu/get_all_enabled_menu_tree",
    tag = "SysMenu",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "获取全部路由菜单树 懒得改名字了", body = SysMenuTreeAll),
    ),
    params(
        ("page_params" = PageParams, Query, description = "分页参数"),
        ("params" = SysMenuSearchReq, Query, description = "查询参数"),
    ),
)]
/// 获取全部路由菜单树 懒得改名字了
pub async fn get_all_enabled_menu_tree(Query(page_params): Query<PageParams>, Query(search_req): Query<SysMenuSearchReq>) -> Res<Vec<SysMenuTreeAll>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_menu::get_all_enabled_menu_tree(db, page_params, search_req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    get,
    path = "/system/menu/get_auth_list",
    tag = "SysMenu",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "获取api与数据库关联列表 授权列表", body = MenuRelated),
    ),
    params(
        ("page_params" = PageParams, Query, description = "分页参数"),
        ("params" = SysMenuSearchReq, Query, description = "查询参数"),
    ),
)]
/// 获取api与数据库关联列表 授权列表
pub async fn get_related_api_and_db(Query(page_params): Query<PageParams>, Query(search_req): Query<SysMenuSearchReq>) -> Res<ListData<MenuRelated>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_menu::get_related_api_and_db(db, page_params, search_req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    get,
    path = "/system/menu/get_routers",
    tag = "SysMenu",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "获取用户路由，用于渲染菜单", body = SysMenuTree),
    )
)]
/// 获取用户路由，用于渲染菜单
pub async fn get_routers(user: Claims) -> Res<Vec<SysMenuTree>> {
    let db = DB.get_or_init(db_conn).await;
    //  获取 用户角色
    let role_id = match service::sys_role::get_current_admin_role(db, &user.id).await {
        Ok(x) => x,
        Err(e) => return Res::with_err(&e.to_string()),
    };

    // 检查是否超管用户
    let res = if CFG.system.super_user.contains(&user.id) {
        service::sys_menu::get_all_router_tree(db).await
    } else {
        service::sys_menu::get_admin_menu_by_role_ids(db, &role_id).await
    };
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
