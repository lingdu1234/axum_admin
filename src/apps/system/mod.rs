// use crate::middleware;
use poem::Route;

mod api;
mod entities;
mod service;

pub fn api() -> Route {
    Route::new()
        .nest("/hello", api::hello())
        .nest("/chacha", api::chacha())
        .nest("/user", api::sys_user()) //用户管理模块
}
