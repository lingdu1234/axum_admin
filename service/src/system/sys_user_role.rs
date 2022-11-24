use anyhow::Result;
use chrono::Local;
use db::system::entities::{sys_user, sys_user_role};
use sea_orm::{sea_query::Expr, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set, TransactionTrait, Value};

// 添加修改用户角色
pub async fn edit_user_role<C>(db: &C, user_id: &str, role_ids: Vec<String>, created_by: &str) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    // 添加用户角色
    sys_user_role::Entity::insert_many(
        role_ids
            .clone()
            .iter()
            .map(|x| sys_user_role::ActiveModel {
                id: Set(scru128::new_string()),
                user_id: Set(user_id.to_string()),
                role_id: Set(x.to_string()),
                created_by: Set(created_by.to_string()),
                created_at: Set(Local::now().naive_local()),
            })
            .collect::<Vec<_>>(),
    )
    .exec(db)
    .await?;

    sys_user::Entity::update_many()
        .col_expr(sys_user::Column::RoleId, Expr::value(Value::String(None)))
        .filter(sys_user::Column::Id.eq(user_id))
        .filter(sys_user::Column::RoleId.is_not_in(role_ids))
        .exec(db)
        .await?;
    Ok(())
}
// 给角色批量添加用户
pub async fn add_role_by_lot_user_ids<C>(db: &C, user_ids: Vec<String>, role_id: String, created_by: &str) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    sys_user_role::Entity::insert_many(
        user_ids
            .iter()
            .map(|x| sys_user_role::ActiveModel {
                id: Set(scru128::new_string()),
                user_id: Set(x.to_string()),
                role_id: Set(role_id.clone()),
                created_by: Set(created_by.to_string()),
                created_at: Set(Local::now().naive_local()),
            })
            .collect::<Vec<_>>(),
    )
    .exec(db)
    .await?;
    Ok(())
}

// 删除用户角色
pub async fn delete_user_role<C>(db: &C, user_id: &str) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    // 先删除用户角色
    sys_user_role::Entity::delete_many().filter(sys_user_role::Column::UserId.eq(user_id)).exec(db).await?;
    Ok(())
}

//  获取用户角色ids
pub async fn get_role_ids_by_user_id<C>(db: &C, user_id: &str) -> Result<Vec<String>>
where
    C: TransactionTrait + ConnectionTrait,
{
    let s = sys_user_role::Entity::find().filter(sys_user_role::Column::UserId.eq(user_id)).all(db).await?;
    let res = s.iter().map(|x| x.role_id.clone()).collect::<Vec<_>>();
    Ok(res)
}

//  获取用户角色ids
pub async fn get_user_ids_by_role_id<C>(db: &C, role_id: &str) -> Result<Vec<String>>
where
    C: TransactionTrait + ConnectionTrait,
{
    let s = sys_user_role::Entity::find().filter(sys_user_role::Column::RoleId.eq(role_id)).all(db).await?;
    let res = s.iter().map(|x| x.user_id.clone()).collect::<Vec<_>>();
    Ok(res)
}

// 批量删除某个角色的多个用户
pub async fn delete_user_role_by_user_ids<C>(db: &C, user_ids: Vec<String>, role_id: Option<String>) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    let mut d = sys_user_role::Entity::delete_many().filter(sys_user_role::Column::UserId.is_in(user_ids));
    if let Some(role_id) = role_id {
        d = d.filter(sys_user_role::Column::RoleId.eq(role_id))
    };

    d.exec(db).await?;
    Ok(())
}
