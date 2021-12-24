use poem::error::BadRequest;
use poem::web::Query;
use poem::{handler, web::Json, Result};

use validator::Validate;

use crate::apps::system::models::sys_menu::MenuResp;
use crate::apps::system::models::RespData;
use crate::apps::system::service;
use crate::utils::jwt::Claims;
use crate::{db_conn, DB};

use super::super::models::sys_menu::{AddReq, DeleteReq, EditReq, SearchReq, SysMenuTree};

/// get_user_by_id 获取用户Id获取用户   
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_by_id(Query(search_req): Query<SearchReq>) -> Result<Json<MenuResp>> {
    search_req.validate().map_err(BadRequest)?;
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_menu::get_by_id(db, search_req).await?;
    Ok(Json(res))
}

/// add 添加
#[handler]
pub async fn add(Json(add_req): Json<AddReq>) -> Result<Json<RespData>> {
    add_req.validate().map_err(BadRequest)?;
    let db = DB.get_or_init(db_conn).await;
    let result = service::sys_menu::add(db, add_req).await?;
    Ok(Json(result))
}

/// delete 完全删除
#[handler]
pub async fn ddelete(Json(delete_req): Json<DeleteReq>) -> Result<Json<RespData>> {
    delete_req.validate().map_err(BadRequest)?;
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_menu::ddelete(db, delete_req).await?;
    Ok(Json(res))
}

// edit 修改
#[handler]
pub async fn edit(Json(edit_req): Json<EditReq>) -> Result<Json<RespData>> {
    edit_req.validate().map_err(BadRequest)?;
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_menu::edit(db, edit_req).await?;
    Ok(Json(res))
}

/// get_all_menu_tree 获取全部菜单树
#[handler]
pub async fn get_all_menu_tree() -> Result<Json<Vec<SysMenuTree>>> {
    let db = DB.get_or_init(db_conn).await;
    let result = service::sys_menu::get_all_menu_tree(db).await?;
    Ok(Json(result))
}

/// 获取用户路由
#[handler]
pub async fn get_routers(user: Claims) -> Result<Json<Vec<SysMenuTree>>> {
    let db = DB.get_or_init(db_conn).await;
    //    获取角色列表
    let all_roles = service::sys_role::get_all(db).await?;
    //  获取 用户角色
    let roles = service::sys_role::get_admin_role(&user.id, all_roles).await?;
    let role_ids = roles.iter().map(|v| v.id.clone()).collect::<Vec<String>>();
    let result = service::sys_menu::get_admin_menu_by_role_ids(db, role_ids).await?;
    Ok(Json(result))
}
