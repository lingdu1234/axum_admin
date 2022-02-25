use std::time;

use chrono::Local;
use db::{
    common::client::{ReqInfo, ResInfo},
    db_conn,
    system::entities::{prelude::SysOperLog, sys_oper_log},
    DB,
};
use poem::{error::BadRequest, Result};
use sea_orm::{sea_query::Expr, ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::utils::ApiUtils::ALL_APIS;

/// add 添加
pub async fn oper_log_add_req(request_id: String, req: ReqInfo) -> Result<()> {
    let db = DB.get_or_init(db_conn).await;

    let operator_type = match req.clone().method.as_str() {
        "GET" => "1",    // 查询
        "POST" => "2",   // 新增
        "PUT" => "3",    // 修改
        "DELETE" => "4", // 删除
        _ => "0",        // 其他
    };
    let all_apis = ALL_APIS.lock().await;
    let req_path = req.path.as_str().replacen("/", "", 1);
    let api_name = all_apis
        .get(&req_path)
        .unwrap_or(&("".to_string()))
        .to_string();
    let now = Local::now().naive_local();
    let add_data = sys_oper_log::ActiveModel {
        oper_id: Set(request_id),
        time_id: Set(now.timestamp()),
        title: Set(api_name),
        business_type: Set("".to_string()),
        method: Set(req.path),
        request_method: Set(req.method),
        operator_type: Set(operator_type.to_string()),
        oper_name: Set(req.user),
        dept_name: Set("".to_string()),
        oper_url: Set(req.ori_path),
        oper_ip: Set(req.client_info.net.ip),
        oper_location: Set(req.client_info.net.location),
        oper_param: Set(req.data),
        url_param: Set(req.query),
        json_result: Set("".to_string()),
        status: Set("1".to_string()),
        error_msg: Set("".to_string()),
        oper_time: Set(now),
    };
    SysOperLog::insert(add_data)
        .exec(db)
        .await
        .map_err(BadRequest)?;

    Ok(())
}

pub async fn oper_log_add_res(request_id: String, res: ResInfo) -> Result<()> {
    tokio::time::sleep(time::Duration::from_secs(5)).await;
    let db = DB.get_or_init(db_conn).await;
    sys_oper_log::Entity::update_many()
        .col_expr(sys_oper_log::Column::JsonResult, Expr::value(res.data))
        .col_expr(sys_oper_log::Column::Status, Expr::value(res.status))
        .col_expr(sys_oper_log::Column::ErrorMsg, Expr::value(res.err_msg))
        .filter(sys_oper_log::Column::OperId.eq(request_id))
        .exec(db)
        .await
        .map_err(BadRequest)?;
    Ok(())
}
