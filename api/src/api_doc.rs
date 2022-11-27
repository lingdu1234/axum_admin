use app_service::service_utils::jwt;
use configs::CFG;
use db::{
    common,
    system::{models as SysModel, prelude::*},
};
use utoipa::{
    openapi::{
        self,
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
        Server,
    },
    Modify, OpenApi,
};

use super::system;

#[derive(OpenApi)]
#[openapi(
    paths(
        // 验证码
        system::common::get_captcha,
        // 服务器信息
        system::common::get_server_info,
        system::common::get_server_info_sse,
        //
        system::sys_api_db::add,
        system::sys_api_db::get_by_id,
        // 
        system::sys_dept::get_sort_list,
        system::sys_dept::get_all,
        system::sys_dept::get_dept_tree,
        system::sys_dept::get_by_id,
        system::sys_dept::add,
        system::sys_dept::edit,
        system::sys_dept::delete,
        // 
        system::sys_dict_data::get_sort_list,
        system::sys_dict_data::get_by_id,
        system::sys_dict_data::get_by_type,
        system::sys_dict_data::add,
        system::sys_dict_data::edit,
        system::sys_dict_data::delete,
        // 
        system::sys_dict_type::get_sort_list,
        system::sys_dict_type::get_by_id,
        system::sys_dict_type::get_all,
        system::sys_dict_type::add,
        system::sys_dict_type::edit,
        system::sys_dict_type::delete,
        // 
        system::sys_job_log::get_sort_list,
        system::sys_job_log::delete,
        system::sys_job_log::clean,
        // 
        system::sys_job::get_sort_list,
        system::sys_job::get_by_id,
        system::sys_job::change_status,
        system::sys_job::run_task_once,
        system::sys_job::add,
        system::sys_job::edit,
        system::sys_job::delete,
        system::sys_job::validate_cron_str,
        // 
        system::sys_login_log::get_sort_list,
        system::sys_login_log::delete,
        system::sys_login_log::clean,
        // 
        system::sys_menu::get_sort_list,
        system::sys_menu::get_by_id,
        system::sys_menu::add,
        system::sys_menu::edit,
        system::sys_menu::update_log_cache_method,
        system::sys_menu::delete,
        system::sys_menu::get_all_enabled_menu_tree,
        system::sys_menu::get_routers,
        system::sys_menu::get_related_api_and_db,
        // 
        system::sys_oper_log::get_sort_list,
        system::sys_oper_log::get_by_id,
        system::sys_oper_log::delete,
        system::sys_oper_log::clean,
        // 
        system::sys_post::get_sort_list,
        system::sys_post::get_by_id,
        system::sys_post::get_all,
        system::sys_post::delete,
        system::sys_post::add,
        system::sys_post::edit,
        // 
        system::sys_role::get_sort_list,
        system::sys_role::get_by_id,
        system::sys_role::get_all,
        system::sys_role::get_role_menu,
        system::sys_role::get_role_dept,
        system::sys_role::add,
        system::sys_role::delete,
        system::sys_role::edit,
        system::sys_role::change_status,
        system::sys_role::set_data_scope,
        //  
        system::sys_update_log::get_all,
        system::sys_update_log::delete,
        system::sys_update_log::add,
        system::sys_update_log::edit,
        // 
        system::sys_user_online::get_sort_list,
        system::sys_user_online::delete,
        system::sys_user_online::log_out,
        // 
        system::sys_user::get_sort_list,
        system::sys_user::get_by_id,
        system::sys_user::get_profile,
        system::sys_user::update_profile,
        system::sys_user::add,
        system::sys_user::edit,
        system::sys_user::delete,
        system::sys_user::get_info,
        system::sys_user::reset_passwd,
        system::sys_user::update_passwd,
        system::sys_user::change_status,
        system::sys_user::change_role,
        system::sys_user::change_dept,
        system::sys_user::fresh_token,
        system::sys_user::update_avatar,
        system::sys_user::login,



    ),
    components(
        schemas(
            common::captcha::CaptchaImage,
            common::res::PageParams,
            jwt::AuthBody,
            // models
            SysModel::server_info::SysInfo,
            SysModel::server_info::Cpu,
            SysModel::server_info::CpuLoad,
            SysModel::server_info::Memory,
            SysModel::server_info::Server,
            SysModel::server_info::Process,
            SysModel::server_info::DiskUsage,
            SysModel::server_info::Network,

            SysModel::sys_api_db::SysApiDbAddEditReq,
            SysModel::sys_api_db::SysApiDbSearchReq,

            SysModel::sys_dept::SysDeptSearchReq,
            SysModel::sys_dept::SysDeptAddReq,
            SysModel::sys_dept::SysDeptDeleteReq,
            SysModel::sys_dept::SysDeptEditReq,
            SysModel::sys_dept::DeptResp,
            SysModel::sys_dept::RespTree,

            SysModel::sys_dict_data::SysDictDataAddReq,
            SysModel::sys_dict_data::SysDictDataDeleteReq,
            SysModel::sys_dict_data::SysDictDataEditReq,
            SysModel::sys_dict_data::SysDictDataSearchReq,

            SysModel::sys_dict_type::SysDictTypeAddReq,
            SysModel::sys_dict_type::SysDictTypeDeleteReq,
            SysModel::sys_dict_type::SysDictTypeEditReq,
            SysModel::sys_dict_type::SysDictTypeSearchReq,

            SysModel::sys_job_log::SysJobLogSearchReq,
            SysModel::sys_job_log::SysJobLogAddReq,
            SysModel::sys_job_log::SysJobLogDeleteReq,
            SysModel::sys_job_log::SysJobLogCleanReq,

            SysModel::sys_job::SysJobSearchReq,
            SysModel::sys_job::SysJobAddReq,
            SysModel::sys_job::SysJobDeleteReq,
            SysModel::sys_job::SysJobEditReq,
            SysModel::sys_job::SysJobStatusReq,
            SysModel::sys_job::SysJobStatusReq,
            SysModel::sys_job::JobId,
            SysModel::sys_job::ValidateReq,
            SysModel::sys_job::ValidateRes,

            SysModel::sys_login_log::SysLoginLogSearchReq,
            SysModel::sys_login_log::SysLoginLogDeleteReq,

            SysModel::sys_menu::SysMenuSearchReq,
            SysModel::sys_menu::SysMenuTree,
            SysModel::sys_menu::MenuResp,
            SysModel::sys_menu::MenuRelated,
            SysModel::sys_menu::UserMenu,
            SysModel::sys_menu::Meta,
            SysModel::sys_menu::SysMenuTreeAll,
            SysModel::sys_menu::SysMenuAddReq,
            SysModel::sys_menu::SysMenuDeleteReq,
            SysModel::sys_menu::SysMenuEditReq,
            SysModel::sys_menu::LogCacheEditReq,

            SysModel::sys_oper_log::SysOperLogSearchReq,
            SysModel::sys_oper_log::SysOperLogDeleteReq,

            SysModel::sys_post::SysPostSearchReq,
            SysModel::sys_post::SysPostAddReq,
            SysModel::sys_post::SysPostEditReq,
            SysModel::sys_post::SysPostDeleteReq,
            SysModel::sys_post::SysPostResp,

            SysModel::sys_role_api::SysRoleApiAddReq,

            SysModel::sys_role::SysRoleSearchReq,
            SysModel::sys_role::SysRoleAddReq,
            SysModel::sys_role::SysRoleDeleteReq,
            SysModel::sys_role::DataScopeReq,
            SysModel::sys_role::SysRoleEditReq,
            SysModel::sys_role::SysRoleStatusReq,
            SysModel::sys_role::UpdateAuthRoleReq,
            SysModel::sys_role::AddOrCancelAuthRoleReq,
            SysModel::sys_role::SysRoleResp,

            SysModel::sys_update_log::SysUpdateLogAddReq,
            SysModel::sys_update_log::SysUpdateLogEditReq,
            SysModel::sys_update_log::SysUpdateLogDeleteReq,

            SysModel::sys_user_online::SysUserOnlineDeleteReq,
            SysModel::sys_user_online::SysUserOnlineSearchReq,

            SysModel::sys_user::SysUserAddReq,
            SysModel::sys_user::SysUserEditReq,
            SysModel::sys_user::UpdateProfileReq,
            SysModel::sys_user::UserResp,
            SysModel::sys_user::UserWithDept,
            SysModel::sys_user::UserInformation,
            SysModel::sys_user::SysUserSearchReq,
            SysModel::sys_user::SysUserDeleteReq,
            SysModel::sys_user::UserLoginReq,
            SysModel::sys_user::UserInfo,
            SysModel::sys_user::ResetPwdReq,
            SysModel::sys_user::UpdatePwdReq,
            SysModel::sys_user::ChangeStatusReq,
            SysModel::sys_user::ChangeRoleReq,
            SysModel::sys_user::ChangeDeptReq,

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
