use app_service::system;
use axum::{extract::Query, Json};
use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    system::{
        models::sys_job_log::{SysJobLogCleanReq, SysJobLogDeleteReq, SysJobLogSearchReq},
        prelude::SysJobLogModel,
    },
    DB,
};

#[utoipa::path(
    get,
    path = "/system/job_log/list",
    tag = "SysJobLog",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "获取定时任务日志列表", body = SysJobLogModel),
    ),
    params(
        ("page_params" = PageParams, Query, description = "分页参数"),
        ("params" = SysJobLogSearchReq, Query, description = "查询参数"),
    ),
)]
/// 获取定时任务日志列表
pub async fn get_sort_list(Query(page_params): Query<PageParams>, Query(req): Query<SysJobLogSearchReq>) -> Res<ListData<SysJobLogModel>> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_job_log::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    delete,
    path = "/system/job_log/delete",
    tag = "SysJobLog",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "删除定时任务日志", body = String)
    ),
    request_body = SysJobLogDeleteReq,
)]
/// 删除定时任务日志
pub async fn delete(Json(req): Json<SysJobLogDeleteReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_job_log::delete(db, req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    delete,
    path = "/system/job_log/clean",
    tag = "SysJobLog",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "清空定时任务日志,id为空全部清除", body = String)
    ),
    request_body = SysJobLogCleanReq,
)]
/// 清空定时任务日志,id为空全部清除
pub async fn clean(Json(req): Json<SysJobLogCleanReq>) -> Res<String> {
    //  数据验证
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_job_log::clean(db, req.job_id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
