use axum::{extract::Query, Json};
use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    system::{
        models::sys_login_log::{SysLoginLogDeleteReq, SysLoginLogSearchReq},
        prelude::SysLoginLogModel,
    },
    DB,
};

use super::super::service;

/// get_list 获取列表
/// page_params 分页参数

pub async fn get_sort_list(Query(page_params): Query<PageParams>, Query(req): Query<SysLoginLogSearchReq>) -> Res<ListData<SysLoginLogModel>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_login_log::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

pub async fn delete(Json(delete_req): Json<SysLoginLogDeleteReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_login_log::delete(db, delete_req).await;
    println!("{:?}", res);
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

pub async fn clean() -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_login_log::clean(db).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
