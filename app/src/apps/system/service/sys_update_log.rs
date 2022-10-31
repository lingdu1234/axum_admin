use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDateTime};
use db::system::{
    entities::sys_update_log,
    models::sys_update_log::{AddReq, EditReq},
};
use sea_orm::{sea_query::Expr, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, Order, QueryFilter, QueryOrder, Set, TransactionTrait};

pub async fn add(db: &DatabaseConnection, req: AddReq, user_id: &str) -> Result<String> {
    let uid = scru128::new_string();
    let now: NaiveDateTime = Local::now().naive_local();
    let insert_data = sys_update_log::ActiveModel {
        id: Set(uid),
        app_version: Set(req.app_version),
        backend_version: Set(req.backend_version),
        title: Set(req.title),
        content: Set(req.content),
        created_at: Set(now),
        updated_at: Set(now),
        updated_by: Set(user_id.to_string()),
        ..Default::default()
    };
    let txn = db.begin().await?;

    sys_update_log::Entity::insert(insert_data).exec(&txn).await?;

    txn.commit().await?;
    Ok("添加成功".to_string())
}

// edit 修改
pub async fn edit(db: &DatabaseConnection, req: EditReq, user_id: &str) -> Result<String> {
    //  检查字典类型是否存在

    let txn = db.begin().await?;
    // 更新
    sys_update_log::Entity::update_many()
        .col_expr(sys_update_log::Column::AppVersion, Expr::value(req.app_version.clone()))
        .col_expr(sys_update_log::Column::BackendVersion, Expr::value(req.backend_version))
        .col_expr(sys_update_log::Column::Title, Expr::value(req.title.clone()))
        .col_expr(sys_update_log::Column::Content, Expr::value(req.content.clone()))
        .col_expr(sys_update_log::Column::UpdatedBy, Expr::value(user_id))
        .col_expr(sys_update_log::Column::UpdatedAt, Expr::value(Local::now().naive_local()))
        .filter(sys_update_log::Column::Id.eq(req.id.clone()))
        .exec(&txn)
        .await?;

    txn.commit().await?;
    Ok("数据更新成功".to_string())
}

/// delete 完全删除
pub async fn soft_delete<C>(db: &C, id: &str) -> Result<String>
where
    C: ConnectionTrait + TransactionTrait,
{
    let s = sys_update_log::Entity::update_many()
        .col_expr(sys_update_log::Column::DeletedAt, Expr::value(Local::now().naive_local()))
        .filter(sys_update_log::Column::Id.eq(id))
        .exec(db)
        .await?;
    match s.rows_affected {
        0 => Err(anyhow!("删除失败,数据不存在")),
        i => Ok(format!("成功删除{}条数据", i)),
    }
}

pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<sys_update_log::Model>> {
    let s = sys_update_log::Entity::find()
        .filter(sys_update_log::Column::DeletedAt.is_null())
        .order_by(sys_update_log::Column::CreatedAt, Order::Desc)
        .all(db)
        .await?;
    Ok(s)
}
