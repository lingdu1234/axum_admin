use anyhow::Result;
use chrono::Local;
use db::system::entities::sys_user_dept;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set, TransactionTrait};

// 添加修改用户角色
pub async fn edit_user_dept<C>(db: &C, user_id: &str, dept_ids: Vec<String>, created_by: &str) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    // 添加用户角色
    sys_user_dept::Entity::insert_many(
        dept_ids
            .clone()
            .iter()
            .map(|x| sys_user_dept::ActiveModel {
                id: Set(scru128::new_string()),
                user_id: Set(user_id.to_string()),
                dept_id: Set(x.to_string()),
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
pub async fn delete_user_dept<C>(db: &C, user_id: &str) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    // 先删除用户角色
    sys_user_dept::Entity::delete_many().filter(sys_user_dept::Column::UserId.eq(user_id)).exec(db).await?;
    Ok(())
}

// 批量删除某个角色的多个用户
pub async fn delete_user_dept_by_user_ids<C>(db: &C, user_ids: Vec<String>) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    sys_user_dept::Entity::delete_many().filter(sys_user_dept::Column::UserId.is_in(user_ids)).exec(db).await?;
    Ok(())
}

pub async fn get_dept_ids_by_user_id<C>(db: &C, user_id: &str) -> Result<Vec<String>>
where
    C: TransactionTrait + ConnectionTrait,
{
    let s = sys_user_dept::Entity::find().filter(sys_user_dept::Column::UserId.eq(user_id)).all(db).await?;
    let res = s.iter().map(|x| x.dept_id.clone()).collect::<Vec<_>>();
    Ok(res)
}
