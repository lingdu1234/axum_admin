use configs::CFG;
use db::{
    common,
    system::{entities, models},
};
use utoipa::{
    openapi::{
        self,
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
        Server,
    },
    Modify, OpenApi,
};

use crate::apps::system::api;

#[derive(OpenApi)]
#[openapi(
    paths(
        // 验证码
        api::common::get_captcha, 
        // 服务器信息
        api::common::get_server_info,
        api::common::get_server_info_sse,
        //
        api::sys_api_db::add,
        api::sys_api_db::get_by_id,
        // 
        api::sys_dept::get_sort_list,
        api::sys_dept::get_all,
        api::sys_dept::get_dept_tree,
        api::sys_dept::get_by_id,
        api::sys_dept::add,
        api::sys_dept::edit,
        api::sys_dept::delete,

    ),
    components(
        schemas(
            common::captcha::CaptchaImage,
            common::res::PageParams,
            // models
            models::server_info::SysInfo,
            models::server_info::Cpu,
            models::server_info::CpuLoad,
            models::server_info::Memory,
            models::server_info::Server,
            models::server_info::Process,
            models::server_info::DiskUsage,
            models::server_info::Network,
            models::sys_api_db::AddEditReq,
            models::sys_api_db::SearchReq,
            models::sys_dept::SearchReq,
            models::sys_dept::AddReq,
            models::sys_dept::DeleteReq,
            models::sys_dept::EditReq,
            models::sys_dept::DeptResp,
            models::sys_dept::RespTree,

            // entities
            entities::sys_dept::Model,
            entities::sys_api_db::Model,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "common", description = "通用api"),
        (name = "SysApiDb", description = "系统-Api Db对应关系"),
        (name = "SysDept", description = "系统-部门"),
    )
)]
pub struct OpenApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        if let Some(schema) = openapi.components.as_mut() {
            schema.add_security_scheme(
                "authorization",
                SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Bearer).bearer_format("Bearer").build()),
            )
        };
        // 定义服务器地址前缀
        openapi.servers = Some(vec![Server::new(&CFG.server.api_prefix)]);
        // 定义openApi相关信息
        openapi.info = utoipa::openapi::InfoBuilder::new()
            .title("Axum Admin OpenApi")
            .version("v0.0.1")
            .description(Some("一个后台管理面板"))
            .license(Some(utoipa::openapi::License::new("MIT")))
            .contact(Some(utoipa::openapi::ContactBuilder::new().name(Some("lingdu")).email(Some("waong2005@126.com")).build()))
            .build();
    }
}

// fn a() {
//     let a:SysInfo
// }
