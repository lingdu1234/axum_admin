use poem::{handler, web::Path};

#[handler]
pub fn say_hello(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}

#[handler]
pub fn say_hello2() -> String {
    "Hello  world, XXXXXXXXXXXXXXXXXXXXXX!".to_string()
}
