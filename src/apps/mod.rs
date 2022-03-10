use configs::CFG;
use poem::{endpoint::StaticFilesEndpoint, get, post, EndpointExt, Route};

use crate::middleware;

pub mod system;
pub mod test;

pub fn api() -> Route {
    Route::new()
        .nest(&CFG.web.upload_url, StaticFilesEndpoint::new(&CFG.web.upload_dir).show_files_listing())
        // 无需授权Api.通用模块
        .nest("/comm", no_auth_api())
        // 系统管理模块
        .nest(
            "/system",
            system::system_api()
                .with(middleware::ApiAuth)
                .with_if(CFG.log.enable_oper_log, middleware::OperLog)
                .with_if(CFG.server.cache_time > 0, middleware::Cache)
                .with(middleware::Ctx),
        )
        //  测试模块
        .nest(
            "/test",
            test::api::test_api()
                .with(middleware::ApiAuth)
                .with_if(CFG.log.enable_oper_log, middleware::OperLog)
                .with(middleware::Ctx),
        )
}

//

pub fn no_auth_api() -> Route {
    Route::new()
        .at("/login", post(system::SysLogin)) // 登录
        .at("/get_captcha", get(system::get_captcha)) // 获取验证码
        .at("/log_out", post(system::log_out)) // 退出登录
}
