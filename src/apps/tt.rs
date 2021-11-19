// use crate::middleware;
use poem::Route;

mod api;
pub mod models;
mod service;

pub fn api() -> Route {
    Route::new()
        .nest("/hello", api::hello())
        .nest("/chacha", api::chacha())
}
