use poem::{delete, get, post, put, Route};
mod common;
mod sys_dept;
mod sys_dict_data;
mod sys_dict_type;
mod sys_menu;
mod sys_post;
mod sys_role;
mod sys_user;

pub use common::get_captcha;
pub use sys_user::login;

pub fn system_api() -> Route {
    Route::new()
        // .at("/login", post(sys_user::login)) //登录
        .nest("/user", sys_user_api()) //用户管理模块
        .nest("/dict/type", sys_dict_type_api()) //字典类型模块
        .nest("/dict/data", sys_dict_data_api()) //字典数据模块
        .nest("/post", sys_post_api()) //岗位模块
        .nest("/dept", sys_dept_api()) //部门模块
        .nest("/role", sys_role_api()) //角色模块
        .nest("/menu", sys_menu_api()) //路由 菜单 模块
}

fn sys_user_api() -> Route {
    Route::new()
        .at("/list", get(sys_user::get_sort_list)) //获取全部用户
        .at("/get_by_id", get(sys_user::get_by_id)) //按id获取用户
        .at("/add", post(sys_user::add)) //添加用户
        .at("/edit", put(sys_user::edit)) //更新用户
        .at("/delete", delete(sys_user::delete)) //硬删除用户
        .at("/get_info", get(sys_user::get_info)) //获取用户信息
}

fn sys_dict_type_api() -> Route {
    Route::new()
        .at("/list", get(sys_dict_type::get_sort_list)) //获取筛选分页
        .at("/get_all", get(sys_dict_type::get_all)) //获取筛选分页
        .at("/get_by_id", get(sys_dict_type::get_by_id)) //按id获取
        .at("/add", post(sys_dict_type::add)) //添加
        .at("/edit", put(sys_dict_type::edit)) //更新
        // .at("/delete", delete(sys_dict_type::delete)) //软删除
        .at("/delete", delete(sys_dict_type::delete)) //硬删除
}

fn sys_dict_data_api() -> Route {
    Route::new()
        .at("/list", get(sys_dict_data::get_sort_list)) //获取筛选分页
        .at("/get_all", get(sys_dict_data::get_all)) //获取筛选分页
        .at("/get_by_id", get(sys_dict_data::get_by_id)) //按id获取
        .at("/get_by_type", get(sys_dict_data::get_by_type)) //按id获取
        .at("/add", post(sys_dict_data::add)) //添加
        .at("/edit", put(sys_dict_data::edit)) //更新
        .at("/delete", delete(sys_dict_data::delete)) //硬删除
}

fn sys_post_api() -> Route {
    Route::new()
        .at("/list", get(sys_post::get_sort_list)) //获取筛选分页
        .at("/get_all", get(sys_post::get_all)) //获取筛选分页
        .at("/get_by_id", get(sys_post::get_by_id)) //按id获取
        .at("/add", post(sys_post::add)) //添加
        .at("/edit", post(sys_post::edit)) //更新
        // .at("/delete", delete(sys_post::delete)) //软删除
        .at("/ddelete", delete(sys_post::delete)) //硬删除
}

fn sys_dept_api() -> Route {
    Route::new()
        .at("/list", get(sys_dept::get_sort_list)) //获取筛选分页
        .at("/get_all", get(sys_dept::get_all)) //获取筛选分页
        .at("/get_dept_tree", get(sys_dept::get_dept_tree)) //获取部门树
        .at("/get_by_id", get(sys_dept::get_by_id)) //按id获取
        .at("/add", post(sys_dept::add)) //添加
        .at("/edit", put(sys_dept::edit)) //更新
        .at("/delete", delete(sys_dept::delete)) //硬删除
}

fn sys_role_api() -> Route {
    Route::new()
        .at("/list", get(sys_role::get_sort_list)) //获取筛选分页
        .at("/get_all", get(sys_role::get_all)) //获取筛选分页
        .at("/get_by_id", get(sys_role::get_by_id)) //按id获取
        .at("/add", post(sys_role::add)) //添加
        .at("/edit", put(sys_role::edit)) //更新
        .at("/update_auth_role", put(sys_role::update_auth_role)) //更新角色授权
        .at("/set_status", post(sys_role::set_status)) //设置状态
        .at("/set_data_scope", put(sys_role::set_data_scope)) //设置数据权限范围
        .at("/delete", delete(sys_role::delete)) //硬删除
        .at("/get_role_menu", get(sys_role::get_role_menu)) //获取角色菜单
        .at("/get_role_dept", get(sys_role::get_role_dept)) //获取角色部门
}

fn sys_menu_api() -> Route {
    Route::new()
        .at("/list", get(sys_menu::get_sort_list)) //获取筛选分页
        .at("/get_by_id", get(sys_menu::get_by_id)) //按id获取
        .at("/add", post(sys_menu::add)) //添加
        .at("/edit", put(sys_menu::edit)) //更新
        .at("/delete", delete(sys_menu::delete)) //硬删除
        //
        .at("/get_all_menu_tree", get(sys_menu::get_all_menu_tree)) //获取全部路由菜单树
        .at("/get_routers", get(sys_menu::get_routers)) //获取用户菜单树
}
