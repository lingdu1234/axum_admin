use poem::error::BadRequest;
use poem::web::Query;
use poem::{handler, web::Json, Result};

use validator::Validate;

use crate::apps::common::models::{ListData, PageParams, Res};
use crate::apps::system::entities::sys_menu;
use crate::apps::system::models::sys_menu::MenuResp;
use crate::apps::system::service;
use crate::utils::jwt::Claims;
use crate::{db_conn, CFG, DB};

use super::super::models::sys_menu::{AddReq, DeleteReq, EditReq, SearchReq, SysMenuTree};

/// get_all_menu_tree 获取全部菜单
#[handler]
pub async fn get_sort_list(
    Query(page_params): Query<PageParams>,
    Query(search_req): Query<SearchReq>,
) -> Result<Json<Res<ListData<sys_menu::Model>>>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_menu::get_sort_list(db, page_params, search_req).await?;
    Ok(Json(Res::with_data(res)))
}

/// get_user_by_id 获取用户Id获取用户   
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_by_id(Query(search_req): Query<SearchReq>) -> Result<Json<Res<MenuResp>>> {
    search_req.validate().map_err(BadRequest)?;
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_menu::get_by_id(db, search_req).await?;
    Ok(Json(Res::with_data(res)))
}

/// add 添加
#[handler]
pub async fn add(Json(req): Json<AddReq>) -> Json<Res<String>> {
    match req.validate() {
        Ok(_) => {}
        Err(e) => return Json(Res::with_err(&e.to_string())),
    }
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_menu::add(db, req).await;
    match res {
        Ok(res) => Json(Res::with_data_msg(res.id, &res.msg)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}

/// delete 完全删除
#[handler]
pub async fn delete(Json(req): Json<DeleteReq>) -> Json<Res<String>> {
    match req.validate() {
        Ok(_) => {}
        Err(e) => return Json(Res::with_err(&e.to_string())),
    }
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_menu::delete(db, req).await;
    match res {
        Ok(res) => Json(Res::with_data_msg(res.id, &res.msg)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}

// edit 修改
#[handler]
pub async fn edit(Json(edit_req): Json<EditReq>) -> Json<Res<String>> {
    match edit_req.validate() {
        Ok(_) => {}
        Err(e) => return Json(Res::with_err(&e.to_string())),
    }
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_menu::edit(db, edit_req).await;
    match res {
        Ok(res) => Json(Res::with_data_msg(res.id, &res.msg)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}

/// get_all_menu_tree 获取全部菜单树
#[handler]
pub async fn get_all_menu_tree() -> Json<Res<Vec<SysMenuTree>>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_menu::get_all_menu_tree(db).await;
    match res {
        Ok(res) => Json(Res::with_data(res)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}
/// 获取用户路由
#[handler]
pub async fn get_routers(user: Claims) -> Result<Json<Res<Vec<SysMenuTree>>>> {
    let db = DB.get_or_init(db_conn).await;
    //    获取角色列表
    let all_roles = service::sys_role::get_all(db).await?;
    //  获取 用户角色
    let roles = service::sys_role::get_admin_role(&user.id, all_roles).await?;
    let role_ids = roles
        .iter()
        .map(|v| v.role_id.clone())
        .collect::<Vec<String>>();
    // 检查是否超管用户
    let res = if CFG.system.super_user.contains(&user.id) {
        service::sys_menu::get_all_menu_tree(db).await
    } else {
        service::sys_menu::get_admin_menu_by_role_ids(db, role_ids).await
    };
    match res {
        Ok(res) => Ok(Json(Res::with_data(res))),
        Err(e) => Ok(Json(Res::with_err(&e.to_string()))),
    }
}
