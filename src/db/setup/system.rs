use crate::apps::system::entities::sys_user;
use sea_orm::sea_query::{self, ColumnDef};
pub use sea_orm::DatabaseConnection;

use super::create_table;

// 创建表格
pub async fn table_init(db: &DatabaseConnection) {
    // 创建用户表
    create_sys_user_table(db).await;
}

/// 创建用户表
async fn create_sys_user_table(db: &DatabaseConnection) {
    let stmt = sea_query::Table::create()
        .table(sys_user::Entity)
        .if_not_exists()
        .col(
            ColumnDef::new(sys_user::Column::Id)
                .string()
                .not_null()
                .primary_key(),
        )
        .col(
            ColumnDef::new(sys_user::Column::UserName)
                .string()
                .not_null(),
        )
        .to_owned();
    // 创建表格
    create_table(db, &stmt)
        .await
        .expect("create_user_table failed");
    // let user1 = sys_user::ActiveModel {
    //     id: Set(scru128::scru128()),
    //     user_name: Set("admin".to_string()),
    //     ..Default::default()
    // };
    // user1.insert(db).await.expect("insert_user_table failed");
}
