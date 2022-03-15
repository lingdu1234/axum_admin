use core::time::Duration;
use std::time::Instant;

use chrono::Local;
use configs::CFG;
use db::{
    common::{ctx::ReqCtx, res::ResJsonString},
    db_conn,
    system::entities::{prelude::SysOperLog, sys_oper_log},
    DB,
};
use poem::{Endpoint, IntoResponse, Middleware, Request, Response, Result};
use sea_orm::{EntityTrait, Set};

use crate::{apps::system::check_user_online, utils::api_utils::ALL_APIS};

/// req上下文注入中间件 同时进行jwt授权验证 以及日志记录
pub struct OperLog;

impl<E: Endpoint> Middleware<E> for OperLog {
    type Output = OperLogEndpoint<E>;

    fn transform(&self, ep: E) -> Self::Output {
        OperLogEndpoint { inner: ep }
    }
}

/// Endpoint for `Tracing` middleware.
pub struct OperLogEndpoint<E> {
    inner: E,
}

#[poem::async_trait]
impl<E: Endpoint> Endpoint for OperLogEndpoint<E> {
    // type Output = E::Output;
    type Output = Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        let req_ctx = match req.extensions().get::<ReqCtx>() {
            Some(x) => x.clone(),
            None => {
                return match self.inner.call(req).await {
                    Ok(res) => Ok(res.into_response()),
                    Err(e) => Err(e),
                };
            }
        };
        // 开始请求数据
        let now = Instant::now();
        let res_end = self.inner.call(req).await;
        let duration = now.elapsed();
        // 请求结束 记录日志
        match res_end {
            Ok(r) => {
                let res = r.into_response();
                let res_ctx = match res.extensions().get::<ResJsonString>() {
                    Some(x) => x.0.clone(),
                    None => "".to_string(),
                };
                oper_log_add(req_ctx, res_ctx, "1".to_string(), "".to_string(), duration).await;
                Ok(res)
            }
            Err(e) => {
                let ee = e.to_string();
                oper_log_add(req_ctx, "".to_string(), "0".to_string(), ee, duration).await;
                Err(e)
            }
        }
    }
}

pub async fn oper_log_add(req: ReqCtx, res: String, status: String, err_msg: String, duration: Duration) {
    tokio::spawn(async move {
        match oper_log_add_fn(req, res, status, err_msg, duration).await {
            Ok(_) => {}
            Err(e) => {
                tracing::info!("日志添加失败：{}", e.to_string());
            }
        };
    });
}

/// add 添加
pub async fn oper_log_add_fn(req: ReqCtx, res: String, status: String, err_msg: String, duration: Duration) -> Result<()> {
    if !CFG.log.enable_oper_log {
        return Ok(());
    }
    let now = Local::now().naive_local();
    // 打印日志
    let req_data = req.clone();
    let res_data = res.clone();
    let err_msg_data = err_msg.clone();
    let duration_data = duration;
    tracing::info!(
        "\n请求路径:{:?}\n完成时间:{:?}\n消耗时间:{:?}微秒 | {:?}毫秒\n请求数据:{:?}\n响应数据:{}\n错误信息:{:?}\n",
        req_data.path.clone(),
        now,
        duration_data.as_micros(),
        duration_data.as_millis(),
        req_data,
        res_data,
        err_msg_data,
    );
    //  判断是否要记录日志
    let apis = ALL_APIS.lock().await;
    let (api_name, is_log) = match apis.get(&req.path) {
        Some(x) => (x.name.clone(), x.is_log),
        None => ("".to_string(), true),
    };
    drop(apis);
    if !is_log {
        return Ok(());
    }

    let d = duration.as_micros() as i64;

    let db = DB.get_or_init(db_conn).await;

    //  获取用户信息
    let (_, m) = check_user_online(Some(db), req.user.token_id.clone()).await;
    let user = match m {
        Some(x) => x,
        None => return Ok(()),
    };

    let operator_type = match req.method.as_str() {
        "GET" => "1",    // 查询
        "POST" => "2",   // 新增
        "PUT" => "3",    // 修改
        "DELETE" => "4", // 删除
        _ => "0",        // 其他
    };

    let add_data = sys_oper_log::ActiveModel {
        oper_id: Set(scru128::scru128_string()),
        time_id: Set(now.timestamp()),
        title: Set(api_name),
        business_type: Set("".to_string()),
        method: Set(req.path),
        request_method: Set(req.method),
        operator_type: Set(operator_type.to_string()),
        oper_name: Set(req.user.name),
        dept_name: Set(user.dept_name),
        oper_url: Set(req.ori_uri),
        oper_ip: Set(user.ipaddr),
        oper_location: Set(user.login_location),
        oper_param: Set(if req.data.len() > 1000 { req.data.split_at(1000).0.to_string() } else { req.data }),
        json_result: Set(if res.len() > 10000 { res.split_at(10000).0.to_string() } else { res }),
        path_param: Set(req.path_params),
        status: Set(status),
        error_msg: Set(err_msg),
        duration: Set(d),
        oper_time: Set(now),
    };
    SysOperLog::insert(add_data).exec(db).await.expect("oper_log_add error");

    Ok(())
}
