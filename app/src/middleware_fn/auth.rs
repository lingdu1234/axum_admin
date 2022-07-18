
use axum::{
    http::{Request, StatusCode},
    response::Response, middleware::Next,
};
use configs::CFG;
use db::common::ctx::ReqCtx;


use crate::utils::ApiUtils;

pub async fn auth_fn_mid<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let ctx = req.extensions().get::<ReqCtx>().expect("ReqCtx not found");
     // 如果是超级用户，则不需要验证权限，直接放行
     if CFG.system.super_user.contains(&ctx.user.id) {
        return Ok(next.run(req).await);
    }
    // 验证api权限，如果不在路由表中，则放行，否则验证权限

    if ApiUtils::is_in(&ctx.path).await {
        if ApiUtils::check_api_permission(&ctx.path, &ctx.method,&ctx.user.id).await {
            return Ok(next.run(req).await);
        } else {
            return  Err(StatusCode::UNAUTHORIZED);
        }
    } else {
        return  Ok(next.run(req).await);
    }

    
}
