use poem::Route;

mod system;
pub mod test;

pub fn api() -> Route {
    Route::new()
        .nest("/test", test::api()) //测试模块
        .nest("/system", system::api()) //系统管理模块
}
