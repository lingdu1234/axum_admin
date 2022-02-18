use chrono::Local;
use db::system::entities::sys_user_role;
use poem::{error::BadRequest, Result};
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set, TransactionTrait};

// 添加修改用户角色
pub async fn edit_user_role<C>(
    db: &C,
    user_id: &str,
    role_ids: Vec<String>,
    c_user_id: String,
) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    // 添加用户角色
    sys_user_role::Entity::insert_many(
        role_ids
            .iter()
            .map(|x| sys_user_role::ActiveModel {
                id: Set(scru128::scru128_string()),
                user_id: Set(user_id.to_string()),
                role_id: Set(x.to_string()),
                created_by: Set(c_user_id.clone()),
                created_at: Set(Local::now().naive_local()),
            })
            .collect::<Vec<_>>(),
    )
    .exec(db)
    .await
    .map_err(BadRequest)?;
    Ok(())
}

// 删除用户角色
pub async fn delete_user_role<C>(db: &C, user_id: &str) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    // 先删除用户角色
    sys_user_role::Entity::delete_many()
        .filter(sys_user_role::Column::UserId.eq(user_id))
        .exec(db)
        .await
        .map_err(BadRequest)?;
    Ok(())
}

//  获取用户角色ids
pub async fn get_role_ids_by_user_id<C>(db: &C, user_id: &str) -> Result<Vec<String>>
where
    C: TransactionTrait + ConnectionTrait,
{
    let s = sys_user_role::Entity::find()
        .filter(sys_user_role::Column::UserId.eq(user_id))
        .all(db)
        .await
        .map_err(BadRequest)?;
    let res = s.iter().map(|x| x.role_id.clone()).collect::<Vec<_>>();
    Ok(res)
}
