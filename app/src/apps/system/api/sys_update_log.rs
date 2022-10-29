use axum::Json;
use db::{
    common::res::Res,
    db_conn,
    system::{
        entities::sys_update_log,
        models::sys_update_log::{AddReq, DeleteReq, EditReq},
    },
    DB,
};

use super::super::service;
use crate::utils::jwt::Claims;

/// add 添加
pub async fn add(Json(req): Json<AddReq>, user: Claims) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_update_log::add(db, req, &user.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// delete 完全删除
pub async fn delete(Json(req): Json<DeleteReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_update_log::soft_delete(db, &req.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

// edit 修改
pub async fn edit(Json(req): Json<EditReq>, user: Claims) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_update_log::edit(db, req, &user.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// get_all 获取全部
pub async fn get_all() -> Res<Vec<sys_update_log::Model>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_update_log::get_all(db).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
