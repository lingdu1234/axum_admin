use poem::{
    error::BadRequest,
    handler,
    web::{Json, Query},
    Result,
};

use validator::Validate;

use crate::apps::system::{
    models::{
        sys_user::{Resp, UserInfo},
        RespData,
    },
    service,
};
use crate::utils::jwt::{AuthBody, Claims};
use crate::{db_conn, DB};

use super::super::models::{
    sys_user::{AddReq, DeleteReq, EditReq, SearchReq, UserLoginReq},
    PageParams,
};

/// get_user_list 获取用户列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_sort_list(
    Query(page_params): Query<PageParams>,
    Query(search_req): Query<SearchReq>,
) -> Result<Json<RespData>> {
    search_req.validate().map_err(BadRequest)?;
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::get_sort_list(db, page_params, search_req).await?;
    Ok(Json(res))
}

/// get_user_by_id 获取用户Id获取用户   
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_by_id_or_name(Query(search_req): Query<SearchReq>) -> Result<Json<Resp>> {
    search_req.validate().map_err(BadRequest)?;
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::get_by_id_or_name(db, search_req).await?;
    Ok(Json(res))
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
pub async fn ddelete(Json(delete_req): Json<DeleteReq>) -> Result<Json<RespData>> {
    delete_req.validate().map_err(BadRequest)?;
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::ddelete(db, delete_req).await?;
    Ok(Json(res))
}

/// delete 软删除
#[handler]
pub async fn delete(Json(delete_req): Json<DeleteReq>) -> Result<Json<RespData>> {
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
pub async fn login(Json(login_req): Json<UserLoginReq>) -> Result<Json<AuthBody>> {
    login_req.validate().map_err(BadRequest)?;
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::login(db, login_req).await?;
    Ok(Json(res))
}
/// 获取用户登录信息
#[handler]
pub async fn get_info(user: Claims) -> Result<Json<UserInfo>> {
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
    for role in roles {
        // role_names.push(role.name);
        role_ids.push(role.id);
    }
    let permissions = service::sys_menu::get_permissions(role_ids.clone()).await;
    // 获取用户菜单信息
    let result = UserInfo {
        user: user_info,
        roles: role_ids,
        permissions,
    };

    Ok(Json(result))
}
