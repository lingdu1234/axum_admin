use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    test::{
        entities::test_data_scope,
        models::test_data_scope::{AddReq, DeleteReq, SearchReq},
    },
    DB,
};
use poem::{
    handler,
    web::{Json, Query},
};

use super::super::service;
use crate::utils::jwt::Claims;

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_sort_list(Query(page_params): Query<PageParams>, Query(req): Query<SearchReq>, user: Claims) -> Res<ListData<test_data_scope::Model>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::test_data_scope::get_sort_list(db, page_params, req, &user.id).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
/// add 添加
#[handler]
pub async fn add(Json(req): Json<AddReq>, user: Claims) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::test_data_scope::add(db, req, &user.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// delete 完全删除
#[handler]
pub async fn delete(Json(req): Json<DeleteReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::test_data_scope::delete(db, req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
