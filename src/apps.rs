use poem::Route;

pub mod system;

pub fn api() -> Route {
    Route::new().nest("/system", system::api()) //系统管理模块
}
