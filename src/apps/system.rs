// use crate::middleware;
use poem::Route;

mod api;
mod service;

pub fn api() -> Route {
    Route::new()
        // .nest("/hello", api::hello().around(middleware::log))
        // .nest("/chacha", api::chacha().with(middleware::Logger))
        .nest("/hello", api::hello())
        .nest("/chacha", api::chacha())
}
