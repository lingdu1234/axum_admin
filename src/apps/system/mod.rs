// use crate::middleware;
use poem::{post, EndpointExt, Route};

use crate::middleware;

mod api;
pub mod entities;
mod models;
mod service;

pub fn system_api() -> Route {
    Route::new()
        .nest("/user", api::sys_user_api()) //用户管理模块
        .at("/login", post(api::login)) //登录
}
