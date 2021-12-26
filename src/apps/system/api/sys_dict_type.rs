use crate::apps::system::service;
use poem::{
    error::BadRequest,
    handler,
    web::{Json, Query},
    Result,
};
use validator::Validate;

use crate::database::{db_conn, DB};

use super::super::models::{
    sys_dict_type::{AddReq, DeleteReq, EditReq, Resp, SearchReq},
    PageParams, RespData,
};

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_sort_list(
    Query(page_params): Query<PageParams>,
    Query(search_req): Query<SearchReq>,
) -> Result<Json<RespData>> {
    search_req.validate().map_err(BadRequest)?;
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_dict_type::get_sort_list(db, page_params, search_req).await?;
    Ok(Json(res))
}

/// add 添加
#[handler]
pub async fn add(Json(add_req): Json<AddReq>) -> Result<Json<RespData>> {
    let db = DB.get_or_init(db_conn).await;
    add_req.validate().map_err(BadRequest)?;
    let res = service::sys_dict_type::add(db, add_req).await?;
    Ok(Json(res))
}

/// delete 完全删除
#[handler]
pub async fn ddelete(Json(delete_req): Json<DeleteReq>) -> Result<Json<RespData>> {
    delete_req.validate().map_err(BadRequest)?;
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_dict_type::ddelete(db, delete_req).await?;
    Ok(Json(res))
}

// edit 修改
#[handler]
pub async fn edit(Json(edit_req): Json<EditReq>) -> Result<Json<RespData>> {
    edit_req.validate().map_err(BadRequest)?;
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_dict_type::edit(db, edit_req).await?;
    Ok(Json(res))
}

/// get_user_by_id 获取用户Id获取用户   
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_by_id(Query(search_req): Query<SearchReq>) -> Result<Json<Resp>> {
    search_req.validate().map_err(BadRequest)?;
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_dict_type::get_by_id(db, search_req).await?;
    Ok(Json(res))
}

/// get_all 获取全部   
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_all() -> Result<Json<Vec<Resp>>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_dict_type::get_all(db).await?;
    Ok(Json(res))
}
