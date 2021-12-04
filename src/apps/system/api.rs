use super::service::sys_user;
use poem::{delete, get, post, Route};

// 导出
pub use sys_user::login;

pub fn sys_user_api() -> Route {
    Route::new()
        .at("/get", get(sys_user::get_sort_list)) //获取全部用户
        .at("/get_by_id", get(sys_user::get_by_id_or_name)) //按id获取用户
        .at("/add", post(sys_user::add)) //添加用户
        .at("/delete", delete(sys_user::delete)) //软删除用户
        .at("/ddelete", delete(sys_user::ddelete)) //硬删除用户
}
