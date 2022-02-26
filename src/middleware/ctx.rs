use bytes::Bytes;
use db::common::ctx::{ReqCtx, UserInfo};
use poem::{http::StatusCode, Body, Endpoint, Error, FromRequest, Middleware, Request, Result};

use crate::utils::jwt::Claims;

/// req上下文注入中间件 同时进行jwt授权验证
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
    type Output = E::Output;
    // type Output = Response;

    async fn call(&self, mut req: Request) -> Result<Self::Output> {
        // 请求信息ctx注入
        let user = match req.extensions().get::<UserInfo>() {
            Some(x) => x.clone(),
            None => match Claims::from_request_without_body(&req).await {
                Err(e) => return Err(e),
                Ok(claims) => {
                    let u = UserInfo {
                        id: claims.id,
                        token_id: claims.token_id,
                        name: claims.name,
                    };
                    req.extensions_mut().insert(u.clone());
                    u
                }
            },
        };
        let ori_uri = req.original_uri().to_string();
        let method = req.method().to_string();
        let path = req.uri().path().replacen("/", "", 1);
        let path_params = req.uri().query().unwrap_or("").to_string();
        let (req_parts, req_body) = req.into_parts();
        let (bytes, body_data) = match get_body_data(req_body).await {
            Err(e) => return Err(e),
            Ok((x, y)) => (x, y),
        };
        let req_ctx = ReqCtx {
            ori_uri,
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
        Err(e) => Err(Error::from_string(e.to_string(), StatusCode::BAD_REQUEST)),
    }
}
