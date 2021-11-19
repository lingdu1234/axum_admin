use poem::web::Data;
use poem::{handler, web::Path};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::apps::tt::models::prelude::*;
// use crate::database;
use crate::models::users;

#[handler]
pub fn say_hello(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}

#[handler]
pub async fn say_hello2(
    data: Data<&DatabaseConnection>,
    data2: Data<&i32>,
    data3: Data<&u32>,
) -> String {
    // let db = database::db_connect().await;
    println!("db==={:?}", data.0);
    println!("data2==={:?}", data2.0);
    println!("data3==={:?}", data3.0);
    let user_list: Vec<users::Model> = Users::find().all(data.0).await.expect("查找失败");
    format!("用户1姓名是===》：{:?}", user_list[0].name)
}
