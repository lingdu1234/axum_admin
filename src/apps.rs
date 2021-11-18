use poem::Route;

mod system;
mod tt;

pub fn api() -> Route {
    Route::new()
        .nest("/tt", tt::api())
        .nest("/system", system::api())
}
