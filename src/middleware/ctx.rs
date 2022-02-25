use core::time::Duration;
use std::time::Instant;

use chrono::Local;
use configs::CFG;
use db::{
    common::{
        ctx::{ReqCtx, UserInfo},
        res::ResJsonString,
    },
    db_conn,
    system::entities::{prelude::SysOperLog, sys_oper_log},
    DB,
};
use poem::{
    http::StatusCode, Body, Endpoint, Error, FromRequest, IntoResponse, Middleware, Request,
    Response, Result,
};
use sea_orm::{EntityTrait, Set};

use crate::{
    apps::system::check_user_online,
    utils::{api_utils::ALL_APIS, jwt::Claims},
};

/// req上下文注入中间件 同时进行jwt授权验证 以及日志记录
pub struct Context;

impl<E: Endpoint> Middleware<E> for Context {
    type Output = ContextEndpoint<E>;

    fn transform(&self, ep: E) -> Self::Output {
        ContextEndpoint { inner: ep }
    }
}

/// Endpoint for `Tracing` middleware.
pub struct ContextEndpoint<E> {
    inner: E,
}

#[poem::async_trait]
impl<E: Endpoint> Endpoint for ContextEndpoint<E> {
    // type Output = E::Output;
    type Output = Response;

    async fn call(&self, mut req: Request) -> Result<Self::Output> {
        // 请求信息ctx注入
        let claims = match Claims::from_request_without_body(&req).await {
            Err(e) => return Err(e),
            Ok(claims) => claims,
        };
        let body = req.take_body();

        let method = req.method().to_string();
        let body_data = match get_body_data(body).await {
            Err(e) => return Err(e),
            Ok(v) => v,
        };

        let ctx_data = match method.clone().as_str() {
            "GET" => req.uri().query().unwrap_or("").to_string(),
            _ => body_data.clone(),
        };
        let req_ctx = ReqCtx {
            ori_uri: req.original_uri().to_string(),
            path: req.uri().path().replacen("/", "", 1),
            method: method.clone(),
            user: UserInfo {
                id: claims.id,
                token_id: claims.token_id,
                name: claims.name,
            },
            data: ctx_data,
        };

        req.extensions_mut().insert(req_ctx.clone());

        if method.as_str() != "GET" && !body_data.clone().is_empty() {
            req.set_body(Body::from(body_data));
        }

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
                tokio::spawn(async move {
                    oper_log_add(req_ctx, res_ctx, "1".to_string(), "".to_string(), duration)
                        .await
                        .expect("oper_log_add_err");
                });

                Ok(res)
            }
            Err(e) => {
                let ee = e.to_string();
                tokio::spawn(async move {
                    oper_log_add(req_ctx, "".to_string(), "0".to_string(), ee, duration)
                        .await
                        .expect("oper_log_add_err");
                });
                Err(e)
            }
        }
    }
}

/// 获取body数据
async fn get_body_data(body: Body) -> Result<String> {
    let bytes = match body.into_bytes().await {
        Ok(bytes) => bytes,
        Err(e) => return Err(Error::from_string(e.to_string(), StatusCode::BAD_REQUEST)),
    };

    match std::str::from_utf8(&bytes) {
        Ok(x) => Ok(x.to_string()),
        Err(e) => return Err(Error::from_string(e.to_string(), StatusCode::BAD_REQUEST)),
    }
}

/// add 添加
pub async fn oper_log_add(
    req: ReqCtx,
    res: String,
    status: String,
    err_msg: String,
    duration: Duration,
) -> Result<()> {
    if !CFG.log.enable_oper_log {
        return Ok(());
    }
    let now = Local::now().naive_local();
    // 打印日志
    let req_data = req.clone();
    let res_data = res.clone();
    let err_msg_data = err_msg.clone();
    let duration_data = duration;
    tokio::spawn(async move {
        tracing::info!(
            "\n请求路径:{:?}\n完成时间:{:?}\n消耗时间:{:?}微秒 | {:?}毫秒\n请求数据:{:?}\n响应数据:{}\n错误信息:{:?}\n",
            req_data.path.clone(),
            now,
            duration_data.as_micros(),duration_data.as_millis(),
            req_data,
            res_data,
            err_msg_data,
        );
    });
    // 当记录日志为操作日志，而且为GET请求时，不记录日志,否则数据要爆炸
    if req.path.clone().contains("oper_log") && req.method.clone() == "GET" {
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
    let all_apis = ALL_APIS.lock().await;
    let req_path = req.path.as_str().replacen('/', "", 1);
    let api_name = all_apis
        .get(&req_path)
        .unwrap_or(&("".to_string()))
        .to_string();

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
        oper_param: Set(if req.data.len() > 1000 {
            req.data.split_at(1000).0.to_string()
        } else {
            req.data
        }),
        json_result: Set(if res.len() > 10000 {
            res.split_at(10000).0.to_string()
        } else {
            res
        }),
        status: Set(status),
        error_msg: Set(err_msg),
        duration: Set(d),
        oper_time: Set(now),
    };
    SysOperLog::insert(add_data)
        .exec(db)
        .await
        .expect("oper_log_add error");

    Ok(())
}
