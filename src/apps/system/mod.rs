// use crate::middleware;
use poem::Route;

mod api;
pub mod entities;
mod models;
mod service;

pub fn api() -> Route {
    Route::new().nest("/user", api::sys_user()) //用户管理模块
}
