use poem::Route;

pub mod system;

pub fn api() -> Route {
    Route::new().nest("/system", system::system_api()) //系统管理模块
}
