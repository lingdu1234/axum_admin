use configs::CFG;
use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    system::{
        entities::sys_menu,
        models::sys_menu::{AddReq, DeleteReq, EditReq, LogCacheEditReq, MenuRelated, MenuResp, SearchReq, SysMenuTree},
    },
    DB,
};
use poem::{
    handler,
    web::{Json, Query},
};

use super::super::service;
use crate::utils::jwt::Claims;

/// get_all_menu_tree 获取全部菜单
#[handler]
pub async fn get_sort_list(Query(page_params): Query<PageParams>, Query(search_req): Query<SearchReq>) -> Res<ListData<sys_menu::Model>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_menu::get_sort_list(db, page_params, search_req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// get_user_by_id 获取用户Id获取用户
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_by_id(Query(search_req): Query<SearchReq>) -> Res<MenuResp> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_menu::get_by_id(db, search_req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// add 添加
#[handler]
pub async fn add(Json(req): Json<AddReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_menu::add(db, req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// delete 完全删除
#[handler]
pub async fn delete(Json(req): Json<DeleteReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_menu::delete(db, &req.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

// edit 修改
#[handler]
pub async fn edit(Json(edit_req): Json<EditReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_menu::edit(db, edit_req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
// update_log_cache_method 修改菜单日志缓存方法
#[handler]
pub async fn update_log_cache_method(Json(edit_req): Json<LogCacheEditReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_menu::update_log_cache_method(db, edit_req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// get_all_menu_tree 获取全部菜单树
#[handler]
pub async fn get_all_enabled_menu_tree() -> Res<Vec<SysMenuTree>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_menu::get_all_enabled_menu_tree(db).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// get_related_api_and_db 获取全部菜单树
#[handler]
pub async fn get_related_api_and_db(Query(page_params): Query<PageParams>, Query(search_req): Query<SearchReq>) -> Res<ListData<MenuRelated>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_menu::get_related_api_and_db(db, page_params, search_req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// 获取用户路由
#[handler]
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
