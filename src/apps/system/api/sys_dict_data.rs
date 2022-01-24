use crate::apps::{
    common::models::{ListData, Res},
    system::{entities::sys_dict_data, service},
};
use poem::{
    error::BadRequest,
    handler,
    web::{Json, Query},
    Result,
};

use crate::apps::common::models::{PageParams, RespData};
use validator::Validate;

use crate::database::{db_conn, DB};

use super::super::models::sys_dict_data::{AddReq, DeleteReq, EditReq, Resp, SearchReq};

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_sort_list(
    Query(page_params): Query<PageParams>,
    Query(req): Query<SearchReq>,
) -> Json<Res<ListData<sys_dict_data::Model>>> {
    match req.validate() {
        Ok(_) => {}
        Err(e) => return Json(Res::with_err(&e.to_string())),
    };
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_dict_data::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => Json(Res::with_data(x)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}

/// add 添加
#[handler]
pub async fn add(Json(req): Json<AddReq>) -> Json<Res<String>> {
    match req.validate() {
        Ok(_) => {}
        Err(e) => return Json(Res::with_err(&e.to_string())),
    };
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_dict_data::add(db, req).await;
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
    let res = service::sys_dict_data::delete(db, req).await;
    match res {
        Ok(x) => Json(Res::with_msg(&x.msg)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}

// edit 修改
#[handler]
pub async fn edit(Json(req): Json<EditReq>) -> Result<Json<RespData>> {
    req.validate().map_err(BadRequest)?;
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_dict_data::edit(db, req).await?;
    Ok(Json(res))
}

/// get_user_by_id 获取用户Id获取用户   
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_by_id(Query(req): Query<SearchReq>) -> Json<Res<sys_dict_data::Model>> {
    match req.validate() {
        Ok(_) => {}
        Err(e) => return Json(Res::with_err(&e.to_string())),
    };
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_dict_data::get_by_id(db, req).await;
    match res {
        Ok(x) => Json(Res::with_data(x)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}

/// get_user_by_id 获取用户Id获取用户   
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_by_type(Query(req): Query<SearchReq>) -> Json<Res<Vec<sys_dict_data::Model>>> {
    match req.validate() {
        Ok(_) => {}
        Err(e) => return Json(Res::with_err(&e.to_string())),
    };
    let db = DB.get_or_init(db_conn).await;
    match service::sys_dict_data::get_by_type(db, req).await {
        Ok(res) => return Json(Res::with_data(res)),
        Err(e) => return Json(Res::with_err(&e.to_string())),
    };
}

/// get_all 获取全部   
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_all() -> Result<Json<Vec<Resp>>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_dict_data::get_all(db).await?;
    Ok(Json(res))
}
