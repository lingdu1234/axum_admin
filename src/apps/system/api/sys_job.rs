use crate::apps::common::models::{ListData, PageParams, Res};
use crate::apps::system::entities::sys_job;
use crate::apps::system::service;
use crate::utils::jwt::Claims;
use poem::{
    error::BadRequest,
    handler,
    web::{Json, Query},
    Result,
};
use validator::Validate;

use crate::database::{db_conn, DB};

use super::super::models::sys_job::{AddReq, DeleteReq, EditReq, SearchReq};

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_sort_list(
    Query(page_params): Query<PageParams>,
    Query(req): Query<SearchReq>,
) -> Json<Res<ListData<sys_job::Model>>> {
    // match req.validate() {
    //     Ok(_) => {}
    //     Err(e) => {
    //         return Json(Res::with_err(&e.to_string()));
    //     }
    // };
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_job::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => Json(Res::with_data(x)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}
/// add 添加
#[handler]
pub async fn add(Json(req): Json<AddReq>, user: Claims) -> Json<Res<String>> {
    // match req.validate() {
    //     Ok(_) => {}
    //     Err(e) => return Json(Res::with_err(&e.to_string())),
    // };
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_job::add(db, req, user.id).await;
    match res {
        Ok(x) => Json(Res::with_msg(&x)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}

/// delete 完全删除
#[handler]
pub async fn delete(Json(req): Json<DeleteReq>) -> Json<Res<String>> {
    // match req.validate() {
    //     Ok(_) => {}
    //     Err(e) => return Json(Res::with_err(&e.to_string())),
    // };
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_job::delete(db, req).await;
    match res {
        Ok(x) => Json(Res::with_msg(&x)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}

// edit 修改
#[handler]
pub async fn edit(Json(edit_req): Json<EditReq>, user: Claims) -> Json<Res<String>> {
    // edit_req.validate().map_err(BadRequest)?;
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_job::edit(db, edit_req, user.id).await;
    match res {
        Ok(x) => Json(Res::with_msg(&x)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}

/// get_user_by_id 获取用户Id获取用户   
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_by_id(Query(req): Query<SearchReq>) -> Json<Res<sys_job::Model>> {
    let id = match req.job_id {
        None => return Json(Res::with_err("id不能为空")),
        Some(x) => x,
    };
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_job::get_by_id(db, id).await;
    match res {
        Ok(x) => Json(Res::with_data(x)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}
