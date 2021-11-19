use poem::web::{Data, Json};
use poem::{handler, web::Path};
use sea_orm::{DatabaseConnection, EntityTrait};

use super::super::entities::prelude::*;
use super::super::entities::sys_user;

#[handler]
pub fn say_hello(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}

#[handler]
pub async fn say_hello2(
    data: Data<&DatabaseConnection>,
    // data2: Data<&i32>,
    // data3: Data<&u32>,
) -> Json<serde_json::Value> {
    println!("db==={:?}", data.0);
    // println!("data2==={:?}", data2.0);
    // println!("data3==={:?}", data3.0);
    let user_list: Vec<sys_user::Model> = SysUser::find().all(data.0).await.expect("查找失败");
    format!("用户1姓名是===》：{:?}", user_list[0].user_name);
    Json(serde_json::json!({
        "code": 0,
        "msg": "success",
        "data": user_list
    }))
    // Json(user_list[0].clone())
}
