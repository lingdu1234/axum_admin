use super::super::apps::system;

// 创建表格
pub async fn db_init() {
    // 系统表格初始化
    system::system_db_migration().await;
}
