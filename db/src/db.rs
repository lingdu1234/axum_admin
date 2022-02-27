use std::time::Duration;

use configs::CFG;
use sea_orm::{entity::prelude::DatabaseConnection, ConnectOptions, Database};
use tokio::sync::OnceCell;
//  异步初始化数据库
pub static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn db_conn() -> DatabaseConnection {
    let mut opt = ConnectOptions::new(CFG.database.link.to_owned());
    opt.max_connections(1000)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .sqlx_logging(false);
    let db = Database::connect(opt).await.expect("数据库打开失败");
    tracing::info!("Database connected");
    db
}
