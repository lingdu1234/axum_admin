use anyhow::Result;
use db::system::{
    entities::{prelude::SysApiDb, sys_api_db},
    models::sys_api_db::SysApiDbAddEditReq,
    prelude::SysApiDbModel,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, TransactionTrait};

/// add 添加

pub async fn add(db: &DatabaseConnection, req: SysApiDbAddEditReq) -> Result<String> {
    let txn = db.begin().await?;
    // 先删除原来的记录
    sys_api_db::Entity::delete_many()
        .filter(sys_api_db::Column::ApiId.eq(req.api_id.clone()))
        .exec(&txn)
        .await?;
    // 添加新的记录
    let mut items: Vec<sys_api_db::ActiveModel> = Vec::new();
    for db in req.dbs {
        items.push(sys_api_db::ActiveModel {
            api_id: Set(req.api_id.clone()),
            db: Set(db),
        });
    }
    if !items.is_empty() {
        sys_api_db::Entity::insert_many(items).exec(&txn).await?;
    }
    txn.commit().await?;
    Ok("添加成功".to_string())
}

/// get_all 获取全部
/// db 数据库连接 使用db.0
pub async fn get_by_id(db: &DatabaseConnection, api_id: &str) -> Result<Vec<SysApiDbModel>> {
    let s = SysApiDb::find().filter(sys_api_db::Column::ApiId.eq(api_id)).all(db).await?;
    Ok(s)
}
