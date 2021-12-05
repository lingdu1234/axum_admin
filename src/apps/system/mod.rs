// use crate::middleware;
use poem::{post, Route};

mod api;
pub mod entities;
mod models;
mod service;

pub fn system_api() -> Route {
    Route::new()
        .at("/login", post(api::login)) //登录
        .nest("/user", api::sys_user_api()) //用户管理模块
        .nest("/dict_type", api::sys_dict_type_api()) //字典类型模块
}
