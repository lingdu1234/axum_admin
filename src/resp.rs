use poem::web::Json;
use serde::{Deserialize, Serialize};

/// 结果返回结构体
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Resp<T> {
//     pub code: u64,
//     pub msg: Option<String>,
//     pub data: Option<T>,
// }

pub fn res_success<T: Serialize>(data: T) -> Json<serde_json::Value> {
    Json(serde_json::json! ({
        "code": 0,
        "msg": "success",
        "data": data,
    }))
}
