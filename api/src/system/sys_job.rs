use app_service::{service_utils::jwt::Claims, system, tasks};
use axum::{extract::Query, Json};
use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    system::{
        models::sys_job::{JobId, SysJobAddReq, SysJobDeleteReq, SysJobEditReq, SysJobSearchReq, SysJobStatusReq, ValidateReq, ValidateRes},
        SysJobModel,
    },
    DB,
};

#[utoipa::path(
    get,
    path = "/system/job/list",
    tag = "SysJob",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "获取定时任务列表", body = SysJobModel),
    ),
    params(
        ("page_params" = PageParams, Query, description = "分页参数"),
        ("params" = SysJobSearchReq, Query, description = "查询参数"),
    ),
)]
/// 获取定时任务列表
pub async fn get_sort_list(Query(page_params): Query<PageParams>, Query(req): Query<SysJobSearchReq>) -> Res<ListData<SysJobModel>> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_job::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    post,
    path = "/system/job/add",
    tag = "SysJob",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "新增定时任务", body = String),
    ),
    request_body = SysJobAddReq,
)]
/// 新增定时任务
pub async fn add(user: Claims, Json(req): Json<SysJobAddReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_job::add(db, req, user.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    delete,
    path = "/system/job/delete",
    tag = "SysJob",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "删除定时任务", body = String),
    ),
    request_body = SysJobDeleteReq,
)]
/// 删除定时任务
pub async fn delete(Json(req): Json<SysJobDeleteReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_job::delete(db, req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    put,
    path = "/system/job/edit",
    tag = "SysJob",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "更新定时任务", body = String),
    ),
    request_body = SysJobEditReq,
)]
/// 更新定时任务
pub async fn edit(user: Claims, Json(edit_req): Json<SysJobEditReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_job::edit(db, edit_req, user.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    get,
    path = "/system/job/get_by_id",
    tag = "SysJob",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "按id获取任务", body = SysJobModel),
    ),
    params(
        ("params" = SysJobSearchReq, Query, description = "查询参数"),
    ),
)]
/// 按id获取任务
pub async fn get_by_id(Query(req): Query<SysJobSearchReq>) -> Res<SysJobModel> {
    let id = match req.job_id {
        None => return Res::with_err("id不能为空"),
        Some(x) => x,
    };
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_job::get_by_id(db, id).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    put,
    path = "/system/job/change_status",
    tag = "SysJob",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "更新定时任务状态", body = String),
    ),
    request_body = SysJobStatusReq,
)]
/// 更新定时任务状态
pub async fn change_status(Json(req): Json<SysJobStatusReq>) -> Res<String> {
    //  数据验证
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_job::set_status(db, req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    put,
    path = "/system/job/run_task_once",
    tag = "SysJob",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "运行一次定时任务", body = String),
    ),
    request_body = JobId,
)]
/// 运行一次定时任务
pub async fn run_task_once(Json(req): Json<JobId>) -> Res<String> {
    tasks::run_once_task(req.job_id, req.task_id, true).await;
    Res::with_msg("任务开始执行")
}

#[utoipa::path(
    post,
    path = "/system/job/validate_cron_str",
    tag = "SysJob",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "验证cron表达式", body = String),
    ),
    request_body = JobId,
)]
/// 验证cron表达式
pub async fn validate_cron_str(Json(req): Json<ValidateReq>) -> Res<ValidateRes> {
    let res = system::sys_job::validate_cron_str(req.cron_str);
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
