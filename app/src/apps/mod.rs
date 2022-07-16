use axum::{
    routing::{get, get_service, post},
    Router,
};
use configs::CFG;
use reqwest::StatusCode;
use tower::layer::layer_fn;
use tower_http::services::ServeDir;

use crate::middleware;

pub mod system;
pub mod test;

pub fn api() -> Router {
    Router::new()
        .nest(
            &CFG.web.upload_url,
            get_service(ServeDir::new(&CFG.web.upload_dir))
                .handle_error(|error: std::io::Error| async move { (StatusCode::INTERNAL_SERVER_ERROR, format!("Unhandled internal error: {}", error)) }),
        )
        // 无需授权Api.通用模块
        .nest("/comm", no_auth_api())
        // 系统管理模块
        .nest(
            "/system",
            system::system_api()
                // .layer(layer_fn(|inner| middleware::ApiAuth { inner }))
                // .layer(layer_fn(|inner| middleware::OperLog { inner }))
                // .layer(layer_fn(|inner| middleware::Cache { inner }))
                // .layer(layer_fn(|inner| middleware::Ctx { inner })),
        )
        //  测试模块
        .nest(
            "/test",
            test::api::test_api()
        //         .layer(layer_fn(|inner| middleware::ApiAuth { inner }))
        //         .layer(layer_fn(|inner| middleware::OperLog { inner }))
        //         .layer(layer_fn(|inner| middleware::Ctx { inner })),
        )
}

//

pub fn no_auth_api() -> Router {
    Router::new()
        // .route("/login", post(system::SysLogin)) // 登录
        // .route("/get_captcha", get(system::get_captcha)) // 获取验证码
        .route("/log_out", post(system::log_out)) // 退出登录
}
