mod common;
mod sys_api_db;
mod sys_dept;
mod sys_dict_data;
mod sys_dict_type;
mod sys_job;
mod sys_job_log;
mod sys_login_log;
mod sys_menu;
mod sys_oper_log;
mod sys_post;
mod sys_role; // 角色管理
mod sys_update_log;
mod sys_user;
mod sys_user_online;

//  重新导出部分api
use axum::{
    routing::{delete, get, post, put},
    Router,
};
pub use common::get_captcha;
pub use sys_user::login;
pub use sys_user_online::log_out;

pub fn system_api() -> Router {
    Router::new()
        // .route("/login", post(sys_user::login)) //登录
        .nest("/user", sys_user_api()) // 用户管理模块
        .nest("/dict/type", sys_dict_type_api()) // 字典类型模块
        .nest("/dict/data", sys_dict_data_api()) // 字典数据模块
        .nest("/post", sys_post_api()) // 岗位模块
        .nest("/dept", sys_dept_api()) // 部门模块
        .nest("/role", sys_role_api()) // 角色模块
        .nest("/menu", sys_menu_api()) // 路由 菜单 模块
        .nest("/login-log", sys_login_log_api()) // 登录日志模块
        .nest("/online", sys_user_online_api()) // 在线用户
        .nest("/job", sys_job_api()) // 定时任务
        .nest("/job_log", sys_job_log_api()) // 定时任务日志
        .nest("/oper_log", sys_oper_log_api()) // 操作日志
        .nest("/api_db", sys_api_db_api()) // 操作日志
        .nest("/monitor", sys_monitor_api()) // 操作日志
        .nest("/update_log", sys_update_log_api()) // 更新日志
}

fn sys_user_api() -> Router {
    Router::new()
        .route("/list", get(sys_user::get_sort_list)) // 获取全部用户
        .route("/get_by_id", get(sys_user::get_by_id)) // 按id获取用户
        .route("/get_profile", get(sys_user::get_profile)) // 按当前获取用户信息
        .route("/update_profile", put(sys_user::update_profile)) // 更新用户信息
        .route("/add", post(sys_user::add)) // 添加用户
        .route("/edit", put(sys_user::edit)) // 更新用户
        .route("/delete", delete(sys_user::delete)) // 硬删除用户
        .route("/get_info", get(sys_user::get_info)) // 获取用户信息
        .route("/reset_passwd", put(sys_user::reset_passwd)) // 重置密码
        .route("/update_passwd", put(sys_user::update_passwd)) // 重置密码
        .route("/change_status", put(sys_user::change_status)) // 修改状态
        .route("/change_role", put(sys_user::change_role)) // 切换角色
        .route("/change_dept", put(sys_user::change_dept)) // 切换部门
        .route("/fresh_token", put(sys_user::fresh_token)) // 修改状态
        .route("/update_avatar", post(sys_user::update_avatar)) // 修改头像
}

fn sys_dict_type_api() -> Router {
    Router::new()
        .route("/list", get(sys_dict_type::get_sort_list)) // 获取筛选分页
        .route("/get_all", get(sys_dict_type::get_all)) // 获取筛选分页
        .route("/get_by_id", get(sys_dict_type::get_by_id)) // 按id获取
        .route("/add", post(sys_dict_type::add)) // 添加
        .route("/edit", put(sys_dict_type::edit)) // 更新
        // .route("/delete", delete(sys_dict_type::delete)) //软删除
        .route("/delete", delete(sys_dict_type::delete)) // 硬删除
}

fn sys_dict_data_api() -> Router {
    Router::new()
        .route("/list", get(sys_dict_data::get_sort_list)) // 获取筛选分页
        .route("/get_all", get(sys_dict_data::get_all)) // 获取筛选分页
        .route("/get_by_id", get(sys_dict_data::get_by_id)) // 按id获取
        .route("/get_by_type", get(sys_dict_data::get_by_type)) // 按id获取
        .route("/add", post(sys_dict_data::add)) // 添加
        .route("/edit", put(sys_dict_data::edit)) // 更新
        .route("/delete", delete(sys_dict_data::delete)) // 硬删除
}

fn sys_post_api() -> Router {
    Router::new()
        .route("/list", get(sys_post::get_sort_list)) // 获取筛选分页
        .route("/get_all", get(sys_post::get_all)) // 获取筛选分页
        .route("/get_by_id", get(sys_post::get_by_id)) // 按id获取
        .route("/add", post(sys_post::add)) // 添加
        .route("/edit", put(sys_post::edit)) // 更新
        // .route("/delete", delete(sys_post::delete)) //软删除
        .route("/delete", delete(sys_post::delete)) // 硬删除
}

fn sys_dept_api() -> Router {
    Router::new()
        .route("/list", get(sys_dept::get_sort_list)) // 获取筛选分页
        .route("/get_all", get(sys_dept::get_all)) // 获取筛选分页
        .route("/get_dept_tree", get(sys_dept::get_dept_tree)) // 获取部门树
        .route("/get_by_id", get(sys_dept::get_by_id)) // 按id获取
        .route("/add", post(sys_dept::add)) // 添加
        .route("/edit", put(sys_dept::edit)) // 更新
        .route("/delete", delete(sys_dept::delete)) // 硬删除
}

fn sys_role_api() -> Router {
    Router::new()
        .route("/list", get(sys_role::get_sort_list)) // 获取筛选分页
        .route("/get_all", get(sys_role::get_all)) // 获取筛选分页
        .route("/get_by_id", get(sys_role::get_by_id)) // 按id获取
        .route("/add", post(sys_role::add)) // 添加
        .route("/edit", put(sys_role::edit)) // 更新
        .route("/change_status", put(sys_role::change_status)) // 设置状态
        .route("/set_data_scope", put(sys_role::set_data_scope)) // 设置数据权限范围
        .route("/delete", delete(sys_role::delete)) // 硬删除
        .route("/get_role_menu", get(sys_role::get_role_menu)) // 获取角色菜单
        .route("/get_role_dept", get(sys_role::get_role_dept)) // 获取角色部门
        // .route("/update_auth_role", put(sys_role::update_auth_role)) // 更新角色授权
        // .route("/cancel_auth_user", put(sys_role::cancel_auth_user)) // 批量用户取消角色授权
        // .route("/add_auth_user", put(sys_role::add_auth_user)) // 批量用户角色授权
        // .route("/get_auth_users_by_role_id", get(sys_role::get_auth_users_by_role_id)) // 获取角色对应用户
        // .route("/get_un_auth_users_by_role_id", get(sys_role::get_un_auth_users_by_role_id))
    // 获取角色对应未授权用户
}

fn sys_menu_api() -> Router {
    Router::new()
        .route("/list", get(sys_menu::get_sort_list)) // 获取筛选分页
        // .route("/get_auth_list", get(sys_menu::get_auth_list)) // 权限查询列表
        .route("/get_by_id", get(sys_menu::get_by_id)) // 按id获取
        .route("/add", post(sys_menu::add)) // 添加
        .route("/edit", put(sys_menu::edit)) // 更新
        .route("/update_log_cache_method", put(sys_menu::update_log_cache_method)) // 更新api缓存方式和日志记录方式
        .route("/delete", delete(sys_menu::delete)) // 硬删除
        .route("/get_all_enabled_menu_tree", get(sys_menu::get_all_enabled_menu_tree)) // 获取全部正常的路由菜单树
        .route("/get_routers", get(sys_menu::get_routers)) // 获取用户菜单树
        .route("/get_auth_list", get(sys_menu::get_related_api_and_db)) // 获取用户菜单树
}

fn sys_login_log_api() -> Router {
    Router::new()
        .route("/list", get(sys_login_log::get_sort_list)) // 获取筛选分页
        .route("/clean", delete(sys_login_log::clean)) // 清空
        .route("/delete", delete(sys_login_log::delete)) // 硬删除
}
fn sys_user_online_api() -> Router {
    Router::new()
        .route("/list", get(sys_user_online::get_sort_list)) // 获取筛选分页
        .route("/delete", delete(sys_user_online::delete)) // 删除
}

fn sys_job_api() -> Router {
    Router::new()
        .route("/list", get(sys_job::get_sort_list)) // 获取筛选分页
        .route("/get_by_id", get(sys_job::get_by_id)) // 按id获取
        .route("/change_status", put(sys_job::change_status)) // 设置状态
        .route("/run_task_once", put(sys_job::run_task_once)) // 设置状态
        .route("/add", post(sys_job::add)) // 添加
        .route("/edit", put(sys_job::edit)) // 更新
        .route("/delete", delete(sys_job::delete)) // 硬删除
        .route("/validate_cron_str", post(sys_job::validate_cron_str)) // 验证cron_str
}

fn sys_job_log_api() -> Router {
    Router::new()
        .route("/list", get(sys_job_log::get_sort_list)) // 获取筛选分页
        // .route("/get_by_id", get(sys_job_log::get_by_id)) // 按id获取
        .route("/clean", delete(sys_job_log::clean)) // 清空
        .route("/delete", delete(sys_job_log::delete)) // 硬删除
}
fn sys_oper_log_api() -> Router {
    Router::new()
        .route("/list", get(sys_oper_log::get_sort_list)) // 获取筛选分页
        .route("/get_by_id", get(sys_oper_log::get_by_id)) // 按id获取
        .route("/clean", delete(sys_oper_log::clean)) // 清空
        .route("/delete", delete(sys_oper_log::delete)) // 硬删除
}
fn sys_api_db_api() -> Router {
    Router::new()
        .route("/get_by_id", get(sys_api_db::get_by_id)) // 按id获取
        .route("/add", post(sys_api_db::add)) // 添加
}
fn sys_monitor_api() -> Router {
    Router::new()
        .route("/server", get(common::get_server_info)) // 服务器信息
        .route("/server-event", get(common::get_server_info_sse)) // 服务器信息
}

fn sys_update_log_api() -> Router {
    Router::new()
        .route("/add", post(sys_update_log::add)) // 添加
        .route("/edit", put(sys_update_log::edit)) // 更新
        .route("/delete", delete(sys_update_log::delete)) // 硬删除
        .route("/get_all", get(sys_update_log::get_all)) // 获取全部
}
