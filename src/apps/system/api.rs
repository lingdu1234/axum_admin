use super::service::sys_user;
use poem::{get, post, Route};

// 导出
pub use sys_user::login;

pub fn sys_user() -> Route {
    Route::new()
        .at("/get", get(sys_user::get_user_list)) //获取全部用户
        .at("/get_by_id", get(sys_user::get_user_by_id_or_name)) //按id获取用户
        .at("/add", post(sys_user::add_user)) //添加用户
}
