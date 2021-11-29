use sea_orm::sea_query::TableCreateStatement;
use sea_orm::{error::*, ConnectionTrait, DatabaseConnection, ExecResult};

pub mod system;

// 创建表格
pub async fn db_init(db: &DatabaseConnection) {
    // 系统表格初始化
    system::table_init(db).await;
}

/// 创建表格
async fn create_table(
    db: &DatabaseConnection,
    stmt: &TableCreateStatement,
) -> Result<ExecResult, DbErr> {
    let builder = db.get_database_backend();
    db.execute(builder.build(stmt)).await
}
