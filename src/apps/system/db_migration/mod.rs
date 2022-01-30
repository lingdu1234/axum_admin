mod table_init;

use crate::database::{db_conn, DB};

pub async fn system_db_migration() {
    let db = DB.get_or_init(db_conn).await;
    // 系统表格初始化
    table_init::database_init(db).await;
}
