use sea_orm::entity::prelude::DatabaseConnection;
use sea_orm::{ConnectOptions, Database};
use std::time::Duration;

pub async fn db_connect() -> DatabaseConnection {
    let mut opt =
        ConnectOptions::new("mysql://root:lingdu515639@127.0.0.1:13306/wk_data".to_owned());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8));

    let db = Database::connect(opt).await.expect("数据库打开失败");
    tracing::info!("Database connected");
    db
}
