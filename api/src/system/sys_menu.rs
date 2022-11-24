use app_service::{service_utils::jwt::Claims, system};
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

/// get_all_menu_tree 获取全部菜单

pub async fn get_sort_list(Query(page_params): Query<PageParams>, Query(search_req): Query<SysMenuSearchReq>) -> Res<ListData<SysMenuModel>> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_menu::get_sort_list(db, page_params, search_req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// get_user_by_id 获取用户Id获取用户
/// db 数据库连接 使用db.0

pub async fn get_by_id(Query(search_req): Query<SysMenuSearchReq>) -> Res<MenuResp> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_menu::get_by_id(db, search_req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// add 添加

pub async fn add(Json(req): Json<SysMenuAddReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_menu::add(db, req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// delete 完全删除

pub async fn delete(Json(req): Json<SysMenuDeleteReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_menu::delete(db, &req.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

// edit 修改

pub async fn edit(Json(edit_req): Json<SysMenuEditReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_menu::edit(db, edit_req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
// update_log_cache_method 修改菜单日志缓存方法
pub async fn update_log_cache_method(Json(edit_req): Json<LogCacheEditReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_menu::update_log_cache_method(db, edit_req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// get_all_menu_tree 获取全部菜单树

pub async fn get_all_enabled_menu_tree(Query(page_params): Query<PageParams>, Query(search_req): Query<SysMenuSearchReq>) -> Res<Vec<SysMenuTreeAll>> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_menu::get_all_enabled_menu_tree(db, page_params, search_req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// get_related_api_and_db 获取全部菜单树
pub async fn get_related_api_and_db(Query(page_params): Query<PageParams>, Query(search_req): Query<SysMenuSearchReq>) -> Res<ListData<MenuRelated>> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_menu::get_related_api_and_db(db, page_params, search_req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// 获取用户路由
pub async fn get_routers(user: Claims) -> Res<Vec<SysMenuTree>> {
    let db = DB.get_or_init(db_conn).await;
    //  获取 用户角色
    let role_id = match system::sys_role::get_current_admin_role(db, &user.id).await {
        Ok(x) => x,
        Err(e) => return Res::with_err(&e.to_string()),
    };

    // 检查是否超管用户
    let res = if CFG.system.super_user.contains(&user.id) {
        system::sys_menu::get_all_router_tree(db).await
    } else {
        system::sys_menu::get_admin_menu_by_role_ids(db, &role_id).await
    };
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
