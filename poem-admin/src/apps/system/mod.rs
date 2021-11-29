// use crate::middleware;
use poem::{post, Route};

mod api;
pub mod entities;
mod models;
mod service;

pub fn api() -> Route {
    Route::new()
        .nest("/user", api::sys_user()) //用户管理模块
        .at("/login", post(api::login)) //登录
}
