use crate::apps::system::service::sys_user;
use poem::{get, Route};

use super::service::{chacha, hello};

pub fn chacha() -> Route {
    Route::new()
        .at("/chacha", get(chacha::say_chacha))
        .at("chacha2", get(chacha::say_chacha2))
}

pub fn hello() -> Route {
    Route::new()
        .at("/:name", get(hello::say_hello))
        .at("/", get(hello::say_hello2))
}

pub fn sys_user() -> Route {
    Route::new().at("/", get(sys_user::add_user)) //添加用户
}
