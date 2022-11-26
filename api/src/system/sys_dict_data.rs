use app_service::{service_utils::jwt::Claims, system};
use axum::{extract::Query, response::IntoResponse, Json};
use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    system::{
        models::sys_dict_data::{SysDictDataAddReq, SysDictDataDeleteReq, SysDictDataEditReq, SysDictDataSearchReq},
        prelude::SysDictDataModel,
    },
    DB,
};

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
pub async fn get_sort_list(Query(page_params): Query<PageParams>, Query(req): Query<SysDictDataSearchReq>) -> Res<ListData<SysDictDataModel>> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_dict_data::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// add 添加
pub async fn add(user: Claims, Json(req): Json<SysDictDataAddReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_dict_data::add(db, req, user.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// delete 完全删除
pub async fn delete(Json(req): Json<SysDictDataDeleteReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_dict_data::delete(db, req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

// edit 修改

pub async fn edit(user: Claims,Json(req): Json<SysDictDataEditReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_dict_data::edit(db, req, user.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// get_user_by_id 获取用户Id获取用户
/// db 数据库连接 使用db.0

pub async fn get_by_id(Query(req): Query<SysDictDataSearchReq>) -> Res<SysDictDataModel> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_dict_data::get_by_id(db, req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// get_user_by_id 获取用户Id获取用户
/// db 数据库连接 使用db.0

pub async fn get_by_type(Query(req): Query<SysDictDataSearchReq>) -> Res<Vec<SysDictDataModel>> {
    let db = DB.get_or_init(db_conn).await;
    match system::sys_dict_data::get_by_type(db, req).await {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// get_all 获取全部
/// db 数据库连接 使用db.0

pub async fn get_all() -> impl IntoResponse {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_dict_data::get_all(db).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
