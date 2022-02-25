use core::time::Duration;
use std::time::Instant;

use bytes::Bytes;
use db::common::client::{ReqInfo, ResInfo};
use headers::HeaderMap;
use poem::{
    http::{method::Method, uri::Uri, version::Version, StatusCode},
    web::RemoteAddr,
    Body, Endpoint, FromRequest, IntoResponse, Middleware, Request, Response, Result,
};

use crate::utils::{self, jwt::Claims};

/// Middleware for [`tracing`](https://crates.io/crates/tracing).
#[derive(Default)]
pub struct Loging;
pub struct LogingEndpoint<E> {
    inner: E,
}

impl<E: Endpoint> Middleware<E> for Loging {
    type Output = LogingEndpoint<E>;

    fn transform(&self, ep: E) -> Self::Output {
        LogingEndpoint { inner: ep }
    }
}

/// Endpoint for `Tracing` middleware.

#[poem::async_trait]
impl<E: Endpoint> Endpoint for LogingEndpoint<E> {
    type Output = Response;

    async fn call(&self, mut req: Request) -> Result<Self::Output> {
        let request_id = scru128::scru128_string();
        let header = req.headers().clone();
        let remote_addr = req.remote_addr().clone();
        let uri = req.uri().clone();
        let ori_uri = req.original_uri().clone();
        let method = req.method().clone();
        let version = req.version().clone();
        let user = match Claims::from_request_without_body(&req).await {
            Ok(claims) => (claims.name.clone(), claims.id.clone()),
            Err(_) => ("".to_string(), "".to_string()),
        };
        let body = req.take_body();
        let req_id = request_id.clone();
        tokio::spawn(async move {
            write_req_log(
                req_id,
                body,
                header,
                remote_addr,
                uri,
                ori_uri,
                method,
                version,
                user,
            )
            .await;
        });
        // 开始处理请求
        let now = Instant::now();
        let res = self.inner.call(req).await;
        let duration = now.elapsed();
        match res {
            Ok(resp) => {
                let re = resp.into_response();
                let (res_parts, res_body) = re.into_parts();
                let (bytes, body_data) = get_body_data(res_body).await.expect("获取body数据失败");
                let r = Response::from_parts(res_parts, Body::from(bytes));
                let req_id = request_id.clone();
                tokio::spawn(async move {
                    write_res_log(req_id, duration, body_data, "1", "").await;
                });

                Ok(r)
            }
            Err(err) => {
                let e = err.to_string();
                let req_id = request_id.clone();
                tokio::spawn(async move {
                    write_res_log(req_id, duration, "".to_string(), "0", &e).await;
                });
                Err(err)
            }
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

async fn write_req_log(
    request_id: String,
    body: Body,
    header: HeaderMap,
    remote_addr: RemoteAddr,
    uri: Uri,
    ori_uri: Uri,
    method: Method,
    version: Version,
    user: (String, String),
) {
    let client_info = utils::get_client_info(header, remote_addr).await;
    let (_, body_data) = get_body_data(body).await.expect("获取body数据失败");

    let req_info = ReqInfo {
        user: user.0,
        user_id: user.1,
        client_info,
        data: body_data,
        path: uri.path().to_string(),
        ori_path: ori_uri.to_string(),
        method: method.to_string(),
        query: uri.query().unwrap_or("").to_string(),
    };
    tracing::info!(
        "\n请求协议:{:#?} \n请求id:{:#?} \n请求日志:{:#?}",
        version,
        request_id,
        req_info
    );
    // match req_info.method.as_str() {
    //     "GET" => {}
    //     _ => {
    super::action::oper_log_add_req(request_id, req_info)
        .await
        .expect("写入请求日志失败");
    //     }
    // };
}

async fn write_res_log(request_id: String, d: Duration, body: String, status: &str, err_msg: &str) {
    let body_s = body.replace("/", "");
    let res_info = ResInfo {
        duration: d.as_millis().to_string(),
        status: status.to_string(),
        data: body_s,
        err_msg: err_msg.to_string(),
    };
    tracing::info!("\n请求id:{:#?} \n响应日志:{:#?}", request_id, res_info);

    super::action::oper_log_add_res(request_id, res_info)
        .await
        .expect("写入响应日志失败");
}
