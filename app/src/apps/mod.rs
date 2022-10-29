use axum::{
    middleware,
    routing::{get, get_service, post},
    Router,
};
use configs::CFG;
use reqwest::StatusCode;
use tower_http::services::ServeDir;

use crate::{
    middleware_fn::{auth::auth_fn_mid, cache::cache_fn_mid, ctx::ctx_fn_mid, oper_log::oper_log_fn_mid},
    utils::jwt::Claims,
};

pub mod system;
pub mod test;

pub fn api() -> Router {
    Router::new()
        // 文件上传api
        .nest(
            &CFG.web.upload_url,
            get_service(ServeDir::new(&CFG.web.upload_dir))
                .handle_error(|error: std::io::Error| async move { (StatusCode::INTERNAL_SERVER_ERROR, format!("Unhandled internal error: {}", error)) }),
        )
        // 无需授权Api.通用模块
        .nest("/comm", no_auth_api())
        // 系统管理模块
        .nest("/system", auth_api())
        //  测试模块
        .nest("/test", test_api())
}

// 无需授权api
pub fn no_auth_api() -> Router {
    Router::new()
        .route("/login", post(system::SysLogin)) // 登录
        .route("/get_captcha", get(system::get_captcha)) // 获取验证码
        .route("/log_out", post(system::log_out)) // 退出登录
}

// 需要授权的api
fn auth_api() -> Router {
    let router = system::system_api();
    let router = match &CFG.log.enable_oper_log {
        true => router.layer(middleware::from_fn(oper_log_fn_mid)),
        false => router,
    };
    let router = match CFG.server.cache_time {
        0 => router,
        _ => router.layer(middleware::from_fn(cache_fn_mid)),
    };

    router
        .layer(middleware::from_fn(auth_fn_mid))
        .layer(middleware::from_fn(ctx_fn_mid))
        .layer(middleware::from_extractor::<Claims>())
}

// 测试api
pub fn test_api() -> Router {
    let router = test::api::test_api();

    let router = match &CFG.log.enable_oper_log {
        true => router.layer(middleware::from_fn(oper_log_fn_mid)),
        false => router,
    };
    router
        .route_layer(middleware::from_fn(auth_fn_mid))
        .layer(middleware::from_fn(ctx_fn_mid))
        .layer(middleware::from_extractor::<Claims>())
}
