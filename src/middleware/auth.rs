use std::task::{Context, Poll};

use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
    Error,
};
use configs::CFG;
use db::common::ctx::ReqCtx;
use futures::future::BoxFuture;
use tower::Service;

use crate::utils::ApiUtils;

/// 菜单授权中间件
#[derive(Clone)]
pub struct Auth<E> {
    pub inner: E,
}

#[axum::async_trait]
impl<E> Service<Request<Body>> for Auth<E>
where
    E: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    E::Future: Send + 'static,
{
    type Response = E::Response;
    type Error = E::Error;
    type Future = E::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    async fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let ctx = req.extensions().get::<ReqCtx>().expect("ReqCtx not found");
        // 如果是超级用户，则不需要验证权限，直接放行
        if CFG.system.super_user.contains(&ctx.user.id) {
            return self.inner.call(req).await;
        }

        // 验证api权限，如果不在路由表中，则放行，否则验证权限

        if ApiUtils::is_in(&ctx.path).await {
            if ApiUtils::check_api_permission(&ctx.path, &ctx.method).await {
                return self.inner.call(req).await;
            } else {
                return Err(Error::new(StatusCode::FORBIDDEN, "你没有权限访问该页面/API"));
            }
        } else {
            return self.inner.call(req).await;
        }
    }
}
