use app_service::system;
use axum::{extract::Query, Json};
use db::{
    common::res::Res,
    db_conn,
    system::{
        models::sys_api_db::{SysApiDbAddEditReq, SysApiDbSearchReq},
        prelude::SysApiDbModel,
    },
    DB,
};

/// add 添加

pub async fn add(Json(req): Json<SysApiDbAddEditReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_api_db::add(db, req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// 按id获取
/// db 数据库连接

pub async fn get_by_id(Query(req): Query<SysApiDbSearchReq>) -> Res<Vec<SysApiDbModel>> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_api_db::get_by_id(db, &req.api_id).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
