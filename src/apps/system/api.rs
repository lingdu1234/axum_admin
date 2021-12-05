use super::service::{sys_dict_type, sys_user};
use poem::{delete, get, post, Route};

// 导出
pub use sys_user::login;

pub fn sys_user_api() -> Route {
    Route::new()
        .at("/get", get(sys_user::get_sort_list)) //获取全部用户
        .at("/get_by_id", get(sys_user::get_by_id_or_name)) //按id获取用户
        .at("/add", post(sys_user::add)) //添加用户
        .at("/edit", post(sys_user::edit)) //更新用户
        .at("/delete", delete(sys_user::delete)) //软删除用户
        .at("/ddelete", delete(sys_user::ddelete)) //硬删除用户
}

pub fn sys_dict_type_api() -> Route {
    Route::new()
        .at("/get", get(sys_dict_type::get_sort_list)) //获取筛选分页
        .at("/get_all", get(sys_dict_type::get_all)) //获取筛选分页
        .at("/get_by_id", get(sys_dict_type::get_by_id)) //按id获取
        .at("/add", post(sys_dict_type::add)) //添加
        .at("/edit", post(sys_dict_type::edit)) //更新
        // .at("/delete", delete(sys_dict_type::delete)) //软删除
        .at("/ddelete", delete(sys_dict_type::ddelete)) //硬删除
}
