use crate::apps::system::{models::RespData, service};
use poem::{
    error::BadRequest,
    handler,
    http::StatusCode,
    web::{Json, Query},
    Error, Result,
};

use serde_json::json;
use validator::Validate;

use crate::database::{db_conn, DB};

use super::super::models::{
    sys_role::{AddReq, DataScopeReq, DeleteReq, EditReq, SearchReq, StatusReq},
    PageParams,
};

/// get_list 获取列表
/// page_params 分页参数
#[handler]
pub async fn get_sort_list(
    Query(page_params): Query<PageParams>,
    Query(search_req): Query<SearchReq>,
) -> Result<Json<RespData>> {
    search_req.validate().map_err(BadRequest)?;
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_role::get_sort_list(db, page_params, search_req).await?;
    Ok(Json(res))
}

/// add 添加
#[handler]
pub async fn add(Json(add_req): Json<AddReq>) -> Result<Json<RespData>> {
    add_req.validate().map_err(BadRequest)?;
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_role::add(db, add_req).await?;
    Ok(Json(res))
}

/// delete 完全删除
#[handler]
pub async fn delete(Json(delete_req): Json<DeleteReq>) -> Result<Json<RespData>> {
    delete_req.validate().map_err(BadRequest)?;
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_role::delete(db, delete_req).await?;
    Ok(Json(res))
}

// edit 修改
#[handler]
pub async fn edit(Json(edit_req): Json<EditReq>) -> Result<Json<RespData>> {
    //  数据验证
    edit_req.validate().map_err(BadRequest)?;
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_role::edit(db, edit_req).await?;
    Ok(Json(res))
}

// set_status 修改状态
#[handler]
pub async fn set_status(Json(status_req): Json<StatusReq>) -> Result<Json<RespData>> {
    //  数据验证
    status_req.validate().map_err(BadRequest)?;
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_role::set_status(db, status_req).await?;
    Ok(Json(res))
}
// set_data_scope 修改数据权限范围
#[handler]
pub async fn set_data_scope(Json(data_scope_req): Json<DataScopeReq>) -> Result<Json<RespData>> {
    //  数据验证
    data_scope_req.validate().map_err(BadRequest)?;
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_role::set_data_scope(db, data_scope_req).await?;
    Ok(Json(res))
}

/// get_user_by_id 获取用户Id获取用户   
#[handler]
pub async fn get_by_id(Query(search_req): Query<SearchReq>) -> Result<Json<RespData>> {
    search_req.validate().map_err(BadRequest)?;
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_role::get_by_id(db, search_req).await?;
    Ok(Json(RespData::with_data(json!(res))))
}

/// get_all 获取全部   
#[handler]
pub async fn get_all() -> Result<Json<RespData>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_role::get_all(db).await?;
    Ok(Json(RespData::with_data(json!(res))))
}

/// get_role_menu 获取角色授权菜单id数组   
#[handler]
pub async fn get_role_menu(Query(search_req): Query<SearchReq>) -> Result<Json<RespData>> {
    search_req.validate().map_err(BadRequest)?;
    match search_req.id {
        None => return Err(Error::from_string("id不能为空", StatusCode::BAD_REQUEST)),
        Some(id) => {
            let res = service::sys_menu::get_permissions(vec![id]).await;
            return Ok(Json(RespData::with_data(json!(res))));
        }
    };
}
