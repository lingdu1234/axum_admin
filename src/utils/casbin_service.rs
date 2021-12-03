use crate::CFG;
use sea_orm::DatabaseConnection;
use sea_orm_casbin_adapter::{casbin::prelude::*, casbin::Result as CasbinResult, SeaOrmAdapter};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct CasbinService {
    pub enforcer: Arc<Enforcer>,
}

impl CasbinService {
    pub async fn new(pool: DatabaseConnection) -> CasbinResult<Self> {
        println!("CasbinService 开始初始化了…………………………………………………………………………………………………………………………………………………………………………………………………………………………………………………………………………………………………………………………………………");
        let m = DefaultModel::from_file(&CFG.casbin.model_file)
            .await
            .unwrap();
        let adpt = SeaOrmAdapter::new_with_pool(pool).await.unwrap();
        let mut e = Enforcer::new(m, adpt).await.unwrap();
        e.enable_log(true);
        Ok(CasbinService {
            enforcer: Arc::new(e),
        })
    }
}
