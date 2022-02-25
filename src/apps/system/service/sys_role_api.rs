use anyhow::Result;
use chrono::Local;
use db::system::{entities::sys_role_api, models::sys_role_api::AddReq};
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
// 添加修改用户角色
pub async fn add_role_api<C>(db: &C, role_apis: Vec<AddReq>, created_by: &str) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    // 添加用户角色
    sys_role_api::Entity::insert_many(
        role_apis
            .iter()
            .map(|x| sys_role_api::ActiveModel {
                id: Set(scru128::scru128_string()),
                role_id: Set(x.role_id.clone()),
                api: Set(x.api.clone()),
                method: Set(x.method.clone()),
                created_by: Set(created_by.to_string()),
                created_at: Set(Local::now().naive_local()),
            })
            .collect::<Vec<_>>(),
    )
    .exec(db)
    .await?;
    Ok(())
}

pub async fn delete_role_api<C>(db: &C, role_ids: Vec<String>) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    sys_role_api::Entity::delete_many()
        .filter(sys_role_api::Column::RoleId.is_in(role_ids))
        .exec(db)
        .await?;
    Ok(())
}

pub async fn get_api_by_role_ids<C>(
    db: &C,
    role_ids: Vec<String>,
) -> Result<Vec<sys_role_api::Model>>
where
    C: TransactionTrait + ConnectionTrait,
{
    let res = sys_role_api::Entity::find()
        .filter(sys_role_api::Column::RoleId.is_in(role_ids))
        .all(db)
        .await?;
    Ok(res)
}
