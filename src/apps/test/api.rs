use poem::{get, Route};

use super::service::hello;

pub fn hello() -> Route {
    Route::new()
        .at("/:name", get(hello::say_hello))
        .at("/", get(hello::say_hello2))
}
