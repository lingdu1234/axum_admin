use poem::{delete, get, post, Route};
pub mod test_data_scope;

pub fn test_api() -> Route {
    Route::new().nest("/data_scope", test_data_scope_api()) // 数据权限测试
}

fn test_data_scope_api() -> Route {
    Route::new()
        .at("/list", get(test_data_scope::get_sort_list)) // 获取筛选分页
        .at("/add", post(test_data_scope::add)) // 添加
        .at("/delete", delete(test_data_scope::delete)) // 硬删除
}
