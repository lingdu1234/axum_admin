use app_service::service_utils::ApiUtils;
use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use configs::CFG;
use db::common::ctx::{ReqCtx, UserInfoCtx};

pub async fn auth_fn_mid(req: Request, next: Next) -> Result<Response, StatusCode> {
    let ctx = req.extensions().get::<ReqCtx>().expect("ReqCtx not found");
    let user = req.extensions().get::<UserInfoCtx>().expect("user not found");
    // 如果是超级用户，则不需要验证权限，直接放行
    if CFG.system.super_user.contains(&user.id) {
        return Ok(next.run(req).await);
    }
    // 验证api权限，如果不在路由表中，则放行，否则验证权限

    if ApiUtils::is_in(&ctx.path).await {
        if ApiUtils::check_api_permission(&ctx.path, &ctx.method, &user.id).await {
            Ok(next.run(req).await)
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    } else {
        Ok(next.run(req).await)
    }
}
