use std::task::{Context, Poll};

use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
    Error,
};
use bytes::Bytes;
use configs::CFG;
use db::common::ctx::{ReqCtx, UserInfo};
use futures::future::BoxFuture;
use tower::Service;

use crate::utils::jwt::Claims;
/// req上下文注入中间件 同时进行jwt授权验证
/// Endpoint for `Tracing` middleware.
pub struct Ctx<E> {
    pub inner: E,
}

#[axum::async_trait]
impl<E> Service<Request<Body>> for Ctx<E>
where
    E: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    E::Future: Send + 'static,
{
    type Response = E::Response;
    type Error = E::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    async fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        // 请求信息ctx注入
        let user = match Claims::from_request_without_body(&req).await {
            Err(e) => return Err(e),
            Ok(claims) => UserInfo {
                id: claims.id,
                token_id: claims.token_id,
                name: claims.name,
            },
        };

        let ori_uri_path = req.original_uri().path().to_string();
        let method = req.method().to_string();
        let path = req.original_uri().path().replacen(&(CFG.server.api_prefix.clone() + "/"), "", 1);
        let path_params = req.uri().query().unwrap_or("").to_string();
        let (req_parts, req_body) = req.into_parts();
        let (bytes, body_data) = match get_body_data(req_body).await {
            Err(e) => return Err(e),
            Ok((x, y)) => (x, y),
        };
        let req_ctx = ReqCtx {
            ori_uri: if path_params.is_empty() { ori_uri_path } else { ori_uri_path + "?" + &path_params },
            path,
            path_params,
            method: method.clone(),
            user,
            data: body_data.clone(),
        };

        let mut req = Request::from_parts(req_parts, Body::from(bytes));
        req.extensions_mut().insert(req_ctx);

        // 开始请求数据
        self.inner.call(req).await
    }
}

/// 获取body数据
async fn get_body_data(body: Body) -> Result<(Bytes, String)> {
    let bytes = match body.into_bytes().await {
        Ok(v) => v,
        Err(e) => return Err(Error::from_string(e.to_string(), StatusCode::BAD_REQUEST)),
    };
    match std::str::from_utf8(&bytes) {
        Ok(x) => {
            let res_data = x.to_string();
            Ok((bytes, res_data))
        }
        Err(_) => Ok((bytes, "该数据无法转输出，可能为blob，binary".to_string())),
    }
}
