use poem::{
    error::BadRequest,
    handler,
    web::{Json, Query},
    Result,
};

use serde_json::json;
use validator::Validate;

use crate::{apps::common::models::ListData, utils::jwt::Claims};
use crate::{
    apps::system::{models::sys_user::UserInfo, service},
    utils::jwt::AuthBody,
};
use crate::{
    apps::{
        common::models::{PageParams, Res, RespData},
        system::models::sys_user::Resp,
    },
    CFG,
};
use crate::{db_conn, DB};

use super::super::models::sys_user::{AddReq, DeleteReq, EditReq, SearchReq, UserLoginReq};

/// get_user_list 获取用户列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_sort_list(
    Query(page_params): Query<PageParams>,
    Query(req): Query<SearchReq>,
) -> Json<Res<ListData<Resp>>> {
    match req.validate() {
        Ok(_) => {}
        Err(e) => return Json(Res::with_err(&e.to_string())),
    };
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => Json(Res::with_data(x)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}

/// get_user_by_id 获取用户Id获取用户   
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_by_id_or_name(Query(search_req): Query<SearchReq>) -> Result<Json<RespData>> {
    search_req.validate().map_err(BadRequest)?;
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::get_by_id_or_name(db, search_req).await?;
    Ok(Json(RespData::with_data(json!(res))))
}

/// add 添加
#[handler]
pub async fn add(Json(add_req): Json<AddReq>) -> Result<Json<RespData>> {
    add_req.validate().map_err(BadRequest)?;
    let db = DB.get_or_init(db_conn).await;
    let result = service::sys_user::add(db, add_req).await?;
    Ok(Json(result))
}

/// delete 完全删除
#[handler]
pub async fn delete(Json(delete_req): Json<DeleteReq>) -> Result<Json<RespData>> {
    delete_req.validate().map_err(BadRequest)?;
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::delete(db, delete_req).await?;
    Ok(Json(res))
}

/// delete 软删除
#[handler]
pub async fn delete_soft(Json(delete_req): Json<DeleteReq>) -> Result<Json<RespData>> {
    delete_req.validate().map_err(BadRequest)?;
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::delete_soft(db, delete_req).await?;
    Ok(Json(res))
}

// edit 修改
#[handler]
pub async fn edit(Json(edit_req): Json<EditReq>) -> Result<Json<RespData>> {
    edit_req.validate().map_err(BadRequest)?;
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::edit(db, edit_req).await?;
    Ok(Json(res))
}

/// 用户登录
#[handler]
pub async fn login(Json(login_req): Json<UserLoginReq>) -> Result<Json<Res<AuthBody>>> {
    match login_req.validate() {
        Ok(_) => {}
        Err(e) => return Ok(Json(Res::with_err(&e.to_string()))),
    }
    let db = DB.get_or_init(db_conn).await;
    match service::sys_user::login(db, login_req).await {
        Ok(res) => return Ok(Json(Res::with_data(res))),
        Err(e) => return Ok(Json(Res::with_err(&e.to_string()))),
    };
}
/// 获取用户登录信息
#[handler]
pub async fn get_info(user: Claims) -> Result<Json<Res<UserInfo>>> {
    let db = DB.get_or_init(db_conn).await;
    //  获取用户信息
    let user_info = service::sys_user::get_by_id_or_name(
        db,
        SearchReq {
            user_id: Some(user.id.clone()),
            ..Default::default()
        },
    )
    .await?;
    //    获取角色列表
    let all_roles = service::sys_role::get_all(db).await?;
    //  获取 用户角色
    let roles = service::sys_role::get_admin_role(&user.id, all_roles).await?;
    // let mut role_names: Vec<String> = Vec::new();
    let mut role_ids: Vec<String> = Vec::new();
    if CFG.system.super_user.contains(&user.id) {
        role_ids = vec!["".to_string()];
    } else {
        for role in roles {
            // role_names.push(role.name);
            role_ids.push(role.role_id);
        }
    }
    // 检查是否超管用户
    let permissions = if CFG.system.super_user.contains(&user.id) {
        vec!["*:*:*".to_string()]
    } else {
        service::sys_menu::get_permissions(role_ids.clone()).await
    };
    // let permissions = service::sys_menu::get_permissions(role_ids.clone()).await;
    // 获取用户菜单信息
    let res = UserInfo {
        user: user_info,
        roles: role_ids,
        permissions,
    };

    Ok(Json(Res::with_data(res)))
}
