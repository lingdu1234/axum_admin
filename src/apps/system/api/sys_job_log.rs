use crate::apps::common::models::{ListData, PageParams, Res};
use crate::apps::system::entities::sys_job_log;
use crate::apps::system::service;
use poem::{
    handler,
    web::{Json, Query},
};

use crate::database::{db_conn, DB};

use super::super::models::sys_job_log::{CleanReq, DeleteReq, SearchReq};

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_sort_list(
    Query(page_params): Query<PageParams>,
    Query(req): Query<SearchReq>,
) -> Json<Res<ListData<sys_job_log::Model>>> {
    // match req.validate() {
    //     Ok(_) => {}
    //     Err(e) => {
    //         return Json(Res::with_err(&e.to_string()));
    //     }
    // };
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_job_log::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => Json(Res::with_data(x)),
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
    let res = service::sys_job_log::delete(db, req).await;
    match res {
        Ok(x) => Json(Res::with_msg(&x)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}

#[handler]
pub async fn clean(Json(req): Json<CleanReq>) -> Json<Res<String>> {
    //  数据验证
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_job_log::clean(db, req.job_id).await;
    match res {
        Ok(x) => Json(Res::with_msg(&x)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}

/// get_user_by_id 获取用户Id获取用户   
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_by_id(Query(req): Query<SearchReq>) -> Json<Res<sys_job_log::Model>> {
    let id = match req.job_log_id {
        None => return Json(Res::with_err("id不能为空")),
        Some(x) => x,
    };
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_job_log::get_by_id(db, id).await;
    match res {
        Ok(x) => Json(Res::with_data(x)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}
