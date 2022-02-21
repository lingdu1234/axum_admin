// use std::sync::Arc;

use configs::CFG;
use db::{db_conn, DB};
use once_cell::sync::Lazy;
// use sea_orm_casbin_adapter::{casbin::prelude::*, SeaOrmAdapter};
// use tokio::sync::{Mutex, OnceCell};

pub static CASBIN_MODEL: Lazy<String> =
    Lazy::new(|| std::fs::read_to_string(&CFG.casbin.model_file).expect("读取casbin模型文件失败"));

// 每次和casbin相关的都会初始化一次casbin，不太好，使用once_cell又有问题
pub async fn get_enforcer(is_init: bool) -> Enforcer {
    println!("CasbinService 开始初始化了………………………………………………………………………………………………………………………………………………………………………………………………………………………");
    let db = DB.get_or_init(db_conn).await;
    let m = DefaultModel::from_str(CASBIN_MODEL.as_str()).await.unwrap();
    let adpt = SeaOrmAdapter::new_with_pool(db.clone(), is_init)
        .await
        .unwrap();
    let mut e = Enforcer::new(m, adpt).await.unwrap();
    e.enable_log(true);
    e
}

// 不知道为什么使用全局一个实例，获取到的数据都是缓存的，不会更新，
// 数据库中的数据是正常的，每次重启程序后就正常一次，更新数据就不正常了，
// 如果按照上面的每次获取实例是正常的 有点迷，数据更新到数据库正常，
// 获取的数据就像是缓存的数据 pub async fn get_enforcer2(is_init: bool) ->
// &'static Arc<Mutex<Enforcer>> {     static CASBIN:
// OnceCell<Arc<Mutex<Enforcer>>> = OnceCell::const_new();     CASBIN
//         .get_or_init(|| async {
//             println!("CasbinService
// 开始初始化了………………………………………………………………………………………………2…………………………………………………………………………
// ……………………………………………");             let db = DB.get_or_init(db_conn).await;
//             let m = DefaultModel::from_file(&CFG.casbin.model_file)
//                 .await
//                 .unwrap();
//             let adpt =
// SeaOrmAdapter::new_with_pool(db.clone(),is_init).await.unwrap();
// let mut e = Enforcer::new(m, adpt).await.unwrap();
// e.enable_log(true);             Arc::new(Mutex::new(e))
//         })
//         .await
// }
