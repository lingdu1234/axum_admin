use poem::Route;

mod system;
pub(crate) mod tt;

pub fn api() -> Route {
    Route::new()
        .nest("/tt", tt::api())
        .nest("/system", system::api())
}
