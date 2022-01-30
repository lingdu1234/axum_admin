use sea_orm::sea_query::TableCreateStatement;
use sea_orm::{error::*, ConnectionTrait, DatabaseConnection, ExecResult};

use super::super::apps::system;

// 创建表格
pub async fn db_init() {
    // 系统表格初始化
    system::system_db_migration().await;
}

/// 创建表格
pub async fn create_table(
    db: &DatabaseConnection,
    stmt: &TableCreateStatement,
) -> Result<ExecResult, DbErr> {
    let builder = db.get_database_backend();
    db.execute(builder.build(stmt)).await
}
