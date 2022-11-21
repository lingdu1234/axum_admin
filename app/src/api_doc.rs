use configs::CFG;
use db::{
    common,
    system::{prelude::*, models},
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
use crate::utils::jwt;

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
        // 
        api::sys_dict_data::get_sort_list,
        api::sys_dict_data::get_by_id,
        api::sys_dict_data::get_by_type,
        api::sys_dict_data::add,
        api::sys_dict_data::edit,
        api::sys_dict_data::delete,
        // 
        api::sys_dict_type::get_sort_list,
        api::sys_dict_type::get_by_id,
        api::sys_dict_type::get_all,
        api::sys_dict_type::add,
        api::sys_dict_type::edit,
        api::sys_dict_type::delete,
        // 
        api::sys_job_log::get_sort_list,
        api::sys_job_log::delete,
        api::sys_job_log::clean,
        // 
        api::sys_job::get_sort_list,
        api::sys_job::get_by_id,
        api::sys_job::change_status,
        api::sys_job::run_task_once,
        api::sys_job::add,
        api::sys_job::edit,
        api::sys_job::delete,
        api::sys_job::validate_cron_str,
        // 
        api::sys_login_log::get_sort_list,
        api::sys_login_log::delete,
        api::sys_login_log::clean,
        // 
        api::sys_menu::get_sort_list,
        api::sys_menu::get_by_id,
        api::sys_menu::add,
        api::sys_menu::edit,
        api::sys_menu::update_log_cache_method,
        api::sys_menu::delete,
        api::sys_menu::get_all_enabled_menu_tree,
        api::sys_menu::get_routers,
        api::sys_menu::get_related_api_and_db,
        // 
        api::sys_oper_log::get_sort_list,
        api::sys_oper_log::get_by_id,
        api::sys_oper_log::delete,
        api::sys_oper_log::clean,
        // 
        api::sys_post::get_sort_list,
        api::sys_post::get_by_id, 
        api::sys_post::get_all, 
        api::sys_post::delete,
        api::sys_post::add,
        api::sys_post::edit,
        // 
        api::sys_role::get_sort_list,
        api::sys_role::get_by_id,
        api::sys_role::get_all,
        api::sys_role::get_role_menu,
        api::sys_role::get_role_dept,
        api::sys_role::add,
        api::sys_role::delete,
        api::sys_role::edit,
        api::sys_role::change_status,
        api::sys_role::set_data_scope,
        //  
        api::sys_update_log::get_all, 
        api::sys_update_log::delete,
        api::sys_update_log::add,
        api::sys_update_log::edit,
        // 
        api::sys_user_online::get_sort_list,
        api::sys_user_online::delete,
        api::sys_user_online::log_out,
        // 
        api::sys_user::get_sort_list,
        api::sys_user::get_by_id,
        api::sys_user::get_profile,
        api::sys_user::update_profile,
        api::sys_user::add,
        api::sys_user::edit,
        api::sys_user::delete,
        api::sys_user::get_info,
        api::sys_user::reset_passwd,
        api::sys_user::update_passwd,
        api::sys_user::change_status,
        api::sys_user::change_role,
        api::sys_user::change_dept,
        api::sys_user::fresh_token,
        api::sys_user::update_avatar,
        api::sys_user::login,



    ),
    components(
        schemas(
            common::captcha::CaptchaImage,
            common::res::PageParams,
            jwt::AuthBody,
            // models
            models::server_info::SysInfo,
            models::server_info::Cpu,
            models::server_info::CpuLoad,
            models::server_info::Memory,
            models::server_info::Server,
            models::server_info::Process,
            models::server_info::DiskUsage,
            models::server_info::Network,

            models::sys_api_db::SysApiDbAddEditReq,
            models::sys_api_db::SysApiDbSearchReq,

            models::sys_dept::SysDeptSearchReq,
            models::sys_dept::SysDeptAddReq,
            models::sys_dept::SysDeptDeleteReq,
            models::sys_dept::SysDeptEditReq,
            models::sys_dept::DeptResp,
            models::sys_dept::RespTree,

            models::sys_dict_data::SysDictDataAddReq,
            models::sys_dict_data::SysDictDataDeleteReq,
            models::sys_dict_data::SysDictDataEditReq,
            models::sys_dict_data::SysDictDataSearchReq,

            models::sys_dict_type::SysDictTypeAddReq,
            models::sys_dict_type::SysDictTypeDeleteReq,
            models::sys_dict_type::SysDictTypeEditReq,
            models::sys_dict_type::SysDictTypeSearchReq,

            models::sys_job_log::SysJobLogSearchReq,
            models::sys_job_log::SysJobLogAddReq,
            models::sys_job_log::SysJobLogDeleteReq,
            models::sys_job_log::SysJobLogCleanReq,

            models::sys_job::SysJobSearchReq,
            models::sys_job::SysJobAddReq,
            models::sys_job::SysJobDeleteReq,
            models::sys_job::SysJobEditReq,
            models::sys_job::SysJobStatusReq,
            models::sys_job::SysJobStatusReq,
            models::sys_job::JobId,
            models::sys_job::ValidateReq,
            models::sys_job::ValidateRes,

            models::sys_login_log::SysLoginLogSearchReq,
            models::sys_login_log::SysLoginLogDeleteReq,

            models::sys_menu::SysMenuSearchReq,
            models::sys_menu::SysMenuTree,
            models::sys_menu::MenuResp,
            models::sys_menu::MenuRelated,
            models::sys_menu::UserMenu,
            models::sys_menu::Meta,
            models::sys_menu::SysMenuTreeAll,
            models::sys_menu::SysMenuAddReq,
            models::sys_menu::SysMenuDeleteReq,
            models::sys_menu::SysMenuEditReq,
            models::sys_menu::LogCacheEditReq,

            models::sys_oper_log::SysOperLogSearchReq,
            models::sys_oper_log::SysOperLogDeleteReq,

            models::sys_post::SysPostSearchReq,
            models::sys_post::SysPostAddReq,
            models::sys_post::SysPostEditReq,
            models::sys_post::SysPostDeleteReq,
            models::sys_post::SysPostResp,

            models::sys_role_api::SysRoleApiAddReq,

            models::sys_role::SysRoleSearchReq,
            models::sys_role::SysRoleAddReq,
            models::sys_role::SysRoleDeleteReq,
            models::sys_role::DataScopeReq,
            models::sys_role::SysRoleEditReq,
            models::sys_role::SysRoleStatusReq,
            models::sys_role::UpdateAuthRoleReq,
            models::sys_role::AddOrCancelAuthRoleReq,
            models::sys_role::SysRoleResp,

            models::sys_update_log::SysUpdateLogAddReq,
            models::sys_update_log::SysUpdateLogEditReq,
            models::sys_update_log::SysUpdateLogDeleteReq,

            models::sys_user_online::SysUserOnlineDeleteReq,
            models::sys_user_online::SysUserOnlineSearchReq,

            models::sys_user::SysUserAddReq,
            models::sys_user::SysUserEditReq,
            models::sys_user::UpdateProfileReq,
            models::sys_user::UserResp,
            models::sys_user::UserWithDept,
            models::sys_user::UserInformation,
            models::sys_user::SysUserSearchReq,
            models::sys_user::SysUserDeleteReq,
            models::sys_user::UserLoginReq,
            models::sys_user::UserInfo,
            models::sys_user::ResetPwdReq,
            models::sys_user::UpdatePwdReq,
            models::sys_user::ChangeStatusReq,
            models::sys_user::ChangeRoleReq,
            models::sys_user::ChangeDeptReq,

            // entities
            SysDeptModel,
            SysApiDbModel,
            SysDictDataModel,
            SysDictTypeModel,
            SysJobModel,
            SysJobLogModel,
            SysMenuModel,
            SysOperLogModel,
            SysPostModel,
            SysRoleApiModel,
            SysRoleDeptModel,
            SysRoleModel,
            SysUpdateLogModel,
            SysUserDeptModel,
            SysUserOnlineModel,
            SysUserPostModel,
            SysUserRoleModel,
            SysUserModel,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "common", description = "通用api"),
        (name = "SysApiDb", description = "系统-Api Db对应关系"),
        (name = "SysDept", description = "系统-部门"),
        (name = "SysDictData", description = "系统-字典数据"),
        (name = "SysDictType", description = "系统-字典类型"),
        (name = "SysJob", description = "系统-定时任务"),
        (name = "SysJobLog", description = "系统-任务日志"),
        (name = "SysLoginLog", description = "系统-任务日志"),
        (name = "SysMenu", description = "系统-菜单管理"),
        (name = "SysOperLog", description = "系统-操作日志"),
        (name = "SysPost", description = "系统-岗位管理"),
        (name = "SysRole", description = "系统-角色管理"),
        (name = "SysUpdateLog", description = "系统-更新日志"),
        (name = "SysUserOnline", description = "系统-在线用户"),
        (name = "SysUser", description = "系统-用户"),
        (name = "SysMonitor", description = "系统-服务器信息"),
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
            .description(Some("一个后台管理面板,返回数据为主要数据的数据结构,一些返回数据采用了泛型,以实际返回数据为准"))
            .license(Some(utoipa::openapi::License::new("MIT apache2")))
            .contact(Some(utoipa::openapi::ContactBuilder::new().name(Some("lingdu")).email(Some("waong2005@126.com")).build()))
            .build();
    }
}

// fn a() {
//     let a:SysInfo
// }
