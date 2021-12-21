
use poem::{
    error::BadRequest,
    handler,
    web::{Json, Query}, Result,
};


use validator::Validate;

use crate::apps::system::service;
use crate::{
    utils::{
        jwt::{AuthBody},
    },
};


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
) -> Result<Json<serde_json::Value>> {
    search_req.validate().map_err(BadRequest)?;
    service::sys_user::get_sort_list(page_params, search_req).await
}

/// get_user_by_id 获取用户Id获取用户   
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_by_id_or_name(
    Query(search_req): Query<SearchReq>,
) -> Result<Json<serde_json::Value>> {
    search_req.validate().map_err(BadRequest)?;
    service::sys_user::get_by_id_or_name(search_req).await
}

/// add 添加
#[handler]
pub async fn add(Json(add_req): Json<AddReq>) -> Result<Json<serde_json::Value>> {
    add_req.validate().map_err(BadRequest)?;
    service::sys_user::add(add_req).await
}

/// delete 完全删除
#[handler]
pub async fn ddelete(Json(delete_req): Json<DeleteReq>) -> Result<Json<serde_json::Value>> {
    delete_req.validate().map_err(BadRequest)?;
    service::sys_user::ddelete(delete_req).await
}

/// delete 软删除
#[handler]
pub async fn delete(Json(delete_req): Json<DeleteReq>) -> Result<Json<serde_json::Value>> {
    delete_req.validate().map_err(BadRequest)?;
    service::sys_user::delete_soft(delete_req).await
}

// edit 修改
#[handler]
pub async fn edit(Json(edit_req): Json<EditReq>) -> Result<Json<serde_json::Value>> {
    edit_req.validate().map_err(BadRequest)?;
    service::sys_user::edit(edit_req).await
}

/// 用户登录
#[handler]
pub async fn login(Json(login_req): Json<UserLoginReq>) -> Result<Json<AuthBody>> {
    login_req.validate().map_err(BadRequest)?;
    service::sys_user::login(login_req).await
}
