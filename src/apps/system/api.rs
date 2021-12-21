use super::service;
use poem::{delete, get, post, Route};
mod sys_user;

pub fn system_api() -> Route {
    Route::new()
        .at("/login", post(sys_user::login)) //登录
        .nest("/user", sys_user_api()) //用户管理模块
        .nest("/dict_type", sys_dict_type_api()) //字典类型模块
}

fn sys_user_api() -> Route {
    Route::new()
        .at("/get", get(sys_user::get_sort_list)) //获取全部用户
        .at("/get_by_id", get(sys_user::get_by_id_or_name)) //按id获取用户
        .at("/add", post(sys_user::add)) //添加用户
        .at("/edit", post(sys_user::edit)) //更新用户
        .at("/delete", delete(sys_user::delete)) //软删除用户
        .at("/ddelete", delete(sys_user::ddelete)) //硬删除用户
}

fn sys_dict_type_api() -> Route {
    Route::new()
        .at("/get", get(service::sys_dict_type::get_sort_list)) //获取筛选分页
        .at("/get_all", get(service::sys_dict_type::get_all)) //获取筛选分页
        .at("/get_by_id", get(service::sys_dict_type::get_by_id)) //按id获取
        .at("/add", post(service::sys_dict_type::add)) //添加
        .at("/edit", post(service::sys_dict_type::edit)) //更新
        // .at("/delete", delete(sys_dict_type::delete)) //软删除
        .at("/ddelete", delete(service::sys_dict_type::ddelete)) //硬删除
}
