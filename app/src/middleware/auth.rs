use configs::CFG;
use db::common::ctx::ReqCtx;
use poem::{http::StatusCode, Endpoint, Error, Middleware, Request, Result};

use crate::utils::ApiUtils;

/// 菜单授权中间件
#[derive(Clone, Debug)]
pub struct Auth;

impl<E: Endpoint> Middleware<E> for Auth {
    type Output = AuthEndpoint<E>;

    fn transform(&self, ep: E) -> Self::Output {
        AuthEndpoint { ep }
    }
}

pub struct AuthEndpoint<E> {
    ep: E,
}

#[poem::async_trait]
impl<E: Endpoint> Endpoint for AuthEndpoint<E> {
    type Output = E::Output;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        let ctx = req.extensions().get::<ReqCtx>().expect("ReqCtx not found");
        // 如果是超级用户，则不需要验证权限，直接放行
        if CFG.system.super_user.contains(&ctx.user.id) {
            return self.ep.call(req).await;
        }

        // 验证api权限，如果不在路由表中，则放行，否则验证权限

        if ApiUtils::is_in(&ctx.path).await {
            if ApiUtils::check_api_permission(&ctx.path, &ctx.method, &ctx.user.id).await {
                return self.ep.call(req).await;
            } else {
                return Err(Error::from_string("你没有权限访问该页面/API", StatusCode::FORBIDDEN));
            }
        } else {
            return self.ep.call(req).await;
        }
    }
}
