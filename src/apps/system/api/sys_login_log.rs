use poem::{
    handler,
    web::{Json, Query},
};
use validator::Validate;

use super::super::models::sys_login_log::{DeleteReq, SearchReq};
use crate::{
    apps::{
        common::models::{ListData, PageParams, Res},
        system::{entities::sys_login_log, service},
    },
    database::{db_conn, DB},
};
/// get_list 获取列表
/// page_params 分页参数
#[handler]
pub async fn get_sort_list(
    Query(page_params): Query<PageParams>,
    Query(req): Query<SearchReq>,
) -> Json<Res<ListData<sys_login_log::Model>>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_login_log::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => Json(Res::with_data(x)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}

#[handler]
pub async fn delete(Json(delete_req): Json<DeleteReq>) -> Json<Res<String>> {
    match delete_req.validate() {
        Ok(_) => {}
        Err(e) => return Json(Res::with_err(&e.to_string())),
    };
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_login_log::delete(db, delete_req).await;
    match res {
        Ok(x) => Json(Res::with_msg(&x)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}

#[handler]
pub async fn clean() -> Json<Res<String>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_login_log::clean(db).await;
    match res {
        Ok(x) => Json(Res::with_msg(&x)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}
