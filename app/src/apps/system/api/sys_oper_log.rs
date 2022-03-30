use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    system::{
        entities::sys_oper_log,
        models::sys_oper_log::{DeleteReq, SearchReq},
    },
    DB,
};
use poem::{
    handler,
    web::{Json, Query},
};

use super::super::service;
/// get_list 获取列表
/// page_params 分页参数
#[handler]
pub async fn get_sort_list(Query(page_params): Query<PageParams>, Query(req): Query<SearchReq>) -> Res<ListData<sys_oper_log::Model>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_oper_log::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// delete 完全删除
#[handler]
pub async fn delete(Json(req): Json<DeleteReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_oper_log::delete(db, req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[handler]
pub async fn clean() -> Res<String> {
    //  数据验证
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_oper_log::clean(db).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// get_user_by_id 获取用户Id获取用户
#[handler]
pub async fn get_by_id(Query(req): Query<SearchReq>) -> Res<sys_oper_log::Model> {
    let id = match req.oper_id {
        None => return Res::with_err("id不能为空"),
        Some(x) => x,
    };
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_oper_log::get_by_id(db, id).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
