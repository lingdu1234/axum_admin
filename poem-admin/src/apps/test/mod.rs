// use crate::middleware;
use poem::Route;

mod api;
pub mod entities; //数据库实体
mod service;

pub fn api() -> Route {
    Route::new().nest("/hello", api::hello())
}
