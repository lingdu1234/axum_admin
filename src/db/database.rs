use sea_orm::entity::prelude::DatabaseConnection;
use sea_orm::{ConnectOptions, Database};
use std::time::Duration;

use super::setup;
use crate::CFG;

pub async fn db_connect() -> DatabaseConnection {
    let mut opt = ConnectOptions::new(CFG.database.link.to_owned());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8));
    let db = Database::connect(opt).await.expect("数据库打开失败");
    tracing::info!("Database connected");
    //  初始化数据库
    setup::db_init(&db).await;
    db
}
