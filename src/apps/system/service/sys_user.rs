use poem::{handler, web::Path};

/// add_user 添加用户
#[handler]
pub fn add_user(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}
