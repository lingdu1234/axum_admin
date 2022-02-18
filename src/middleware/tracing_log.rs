use std::time::Instant;

use bytes::Bytes;
use db::common::client::{ReqInfo, ResInfo};
use poem::{
    http::StatusCode, Body, Endpoint, FromRequest, IntoResponse, Request, Response, Result,
};

use crate::{
    apps::system,
    utils::{self, jwt::Claims},
};

pub async fn tracing_log<E: Endpoint>(next: E, req: Request) -> Result<Response> {
    let (req_parts, req_body) = req.into_parts();
    let (req_bytes, req_data) = get_body_data(req_body).await.unwrap();
    let reqq = Request::from_parts(req_parts, Body::from(req_bytes));
    let client_info = utils::get_client_info(&reqq).await;
    let (user, user_id) = match Claims::from_request_without_body(&reqq).await {
        Ok(claims) => (claims.name.clone(), claims.id.clone()),
        Err(_) => ("".to_string(), "".to_string()),
    };
    let req_info = ReqInfo {
        path: reqq.uri().path().to_string(),
        ori_path: reqq.original_uri().to_string(),
        method: reqq.method().to_string(),
        user,
        user_id,
        client_info: client_info.clone(),
        data: req_data,
        query: reqq.uri().query().unwrap_or("").to_string(),
    };
    // 开始处理请求
    let now = Instant::now();
    let res = next.call(reqq).await;
    let duration = now.elapsed();

    match res {
        Ok(resp) => {
            let re = resp.into_response();
            let (res_parts, res_body) = re.into_parts();
            let (bytes, res_data) = get_body_data(res_body).await.unwrap();
            let r = Response::from_parts(res_parts, Body::from(bytes));
            let res_info = ResInfo {
                duration: duration.as_millis().to_string(),
                status: "1".to_string(),
                data: res_data.clone(),
                err_msg: "".to_string(),
            };
            tokio::spawn(async move {
                write_tracing_log(req_info, res_info).await;
            });

            Ok(r)
        }
        Err(err) => {
            let res_info = ResInfo {
                duration: duration.as_millis().to_string(),
                status: "0".to_string(),
                data: "".to_string(),
                err_msg: err.to_string(),
            };
            // tracing::info!("\n请求信息:{:#?} \n响应信息:{:#?}", req_info, res_info);
            tokio::spawn(async move {
                write_tracing_log(req_info, res_info).await;
            });
            Err(err)
        }
    }
}

async fn get_body_data(body: Body) -> Result<(Bytes, String), (StatusCode, String)> {
    let bytes = match body.into_bytes().await {
        Ok(bytes) => bytes,
        Err(e) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read  body: {}", e),
            ));
        }
    };

    match std::str::from_utf8(&bytes) {
        Ok(x) => {
            let res_data = x.to_string();
            return Ok((bytes, res_data));
        }
        Err(e) => return Err((StatusCode::BAD_REQUEST, format!("响应数据解析错误: {}", e))),
    };
}

async fn write_tracing_log(req_info: ReqInfo, res_info: ResInfo) {
    tracing::info!("\n请求信息:{:#?} \n响应信息:{:#?}", req_info, res_info);
    match req_info.method.as_str() {
        "GET" => {}
        _ => {
            system::sys_oper_log_add(req_info, res_info)
                .await
                .expect("写入日志失败");
        }
    };
}
