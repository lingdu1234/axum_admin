use std::sync::Arc;

use crate::{
    database::{db_conn, DB},
    CFG,
};
use sea_orm_casbin_adapter::{casbin::prelude::*, SeaOrmAdapter};
use tokio::sync::{Mutex, OnceCell};

//  异步初始化CASBIN
pub static CASBIN: OnceCell<Arc<Mutex<Enforcer>>> = OnceCell::const_new();

pub async fn get_enforcer() -> Arc<Mutex<Enforcer>> {
    println!("CasbinService 开始初始化了…………………………………………………………………………………………………………………………………………………………………………………………………………………………………………………………………………………………………………………………………………");
    let db = DB.get_or_init(db_conn).await;
    let m = DefaultModel::from_file(&CFG.casbin.model_file)
        .await
        .unwrap();
    let adpt = SeaOrmAdapter::new_with_pool(db.clone()).await.unwrap();
    let mut e = Enforcer::new(m, adpt).await.unwrap();
    e.enable_log(true);
    Arc::new(Mutex::new(e))
}
