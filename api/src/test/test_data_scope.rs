use app_service::{service_utils::jwt::Claims, test};
use axum::{extract::Query, Json};
use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    test::{
        models::test_data_scope::{AddReq, DeleteReq, SearchReq},
        prelude::TestDataScopeModel,
    },
    DB,
};

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0

pub async fn get_sort_list(user: Claims,Query(page_params): Query<PageParams>, Query(req): Query<SearchReq>) -> Res<ListData<TestDataScopeModel>> {
    let db = DB.get_or_init(db_conn).await;
    let res = test::test_data_scope::get_sort_list(db, page_params, req, &user.id).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
/// add 添加

pub async fn add(user: Claims,Json(req): Json<AddReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = test::test_data_scope::add(db, req, &user.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

pub async fn delete(Json(req): Json<DeleteReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = test::test_data_scope::delete(db, req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
