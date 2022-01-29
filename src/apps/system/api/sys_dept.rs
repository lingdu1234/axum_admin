use crate::apps::common::models::{ListData, PageParams, Res};
use crate::apps::system::entities::sys_dept;
use crate::apps::system::models::sys_dept::RespTree;
use crate::apps::system::service;
use crate::utils::jwt::Claims;
use poem::{
    handler,
    web::{Json, Query},
};
use validator::Validate;

use crate::database::{db_conn, DB};

use super::super::models::sys_dept::{AddReq, DeleteReq, DeptResp, EditReq, SearchReq};

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_sort_list(
    Query(page_params): Query<PageParams>,
    Query(req): Query<SearchReq>,
) -> Json<Res<ListData<sys_dept::Model>>> {
    match req.validate() {
        Ok(_) => {}
        Err(e) => {
            return Json(Res::with_err(&e.to_string()));
        }
    };
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_dept::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => Json(Res::with_data(x)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}
/// add 添加
#[handler]
pub async fn add(Json(req): Json<AddReq>, user: Claims) -> Json<Res<String>> {
    match req.validate() {
        Ok(_) => {}
        Err(e) => return Json(Res::with_err(&e.to_string())),
    };
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_dept::add(db, req, user.id).await;
    match res {
        Ok(x) => Json(Res::with_data_msg(x.id, &x.msg)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}

/// delete 完全删除
#[handler]
pub async fn delete(Json(req): Json<DeleteReq>) -> Json<Res<String>> {
    match req.validate() {
        Ok(_) => {}
        Err(e) => return Json(Res::with_err(&e.to_string())),
    };
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_dept::delete(db, req).await;
    match res {
        Ok(x) => Json(Res::with_msg(&x)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}

// edit 修改
#[handler]
pub async fn edit(Json(req): Json<EditReq>, user: Claims) -> Json<Res<String>> {
    match req.validate() {
        Ok(_) => {}
        Err(e) => return Json(Res::with_err(&e.to_string())),
    };
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_dept::edit(db, req, user.id).await;
    match res {
        Ok(x) => Json(Res::with_msg(&x)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}

/// get_user_by_id 获取用户Id获取用户   
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_by_id(Query(req): Query<SearchReq>) -> Json<Res<DeptResp>> {
    match req.validate() {
        Ok(_) => {}
        Err(e) => return Json(Res::with_err(&e.to_string())),
    };
    let db = DB.get_or_init(db_conn).await;
    if let Some(x) = req.dept_id {
        let res = service::sys_dept::get_by_id(db, x).await;
        match res {
            Ok(x) => Json(Res::with_data(x)),
            Err(e) => Json(Res::with_err(&e.to_string())),
        }
    } else {
        Json(Res::with_err("参数错误"))
    }
}

/// get_all 获取全部   
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_all() -> Json<Res<Vec<DeptResp>>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_dept::get_all(db).await;
    match res {
        Ok(x) => Json(Res::with_data(x)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}

#[handler]
pub async fn get_dept_tree() -> Json<Res<Vec<RespTree>>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_dept::get_dept_tree(db).await;
    match res {
        Ok(x) => Json(Res::with_data(x)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}
