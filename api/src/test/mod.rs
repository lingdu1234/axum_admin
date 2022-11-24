mod test_data_scope;

use axum::{
    routing::{delete, get, post},
    Router,
};

pub fn test_api() -> Router {
    Router::new().nest("/data_scope", test_data_scope_api()) // 数据权限测试
}

fn test_data_scope_api() -> Router {
    Router::new()
        .route("/list", get(test_data_scope::get_sort_list)) // 获取筛选分页
        .route("/add", post(test_data_scope::add)) // 添加
        .route("/delete", delete(test_data_scope::delete)) // 硬删除
}
