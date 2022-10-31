use anyhow::Result;
use chrono::Local;
use db::system::{entities::sys_role_api, models::sys_role_api::AddReq};
use sea_orm::{sea_query::Expr, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set, TransactionTrait, UpdateResult};
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
                id: Set(scru128::new_string()),
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
    sys_role_api::Entity::delete_many().filter(sys_role_api::Column::RoleId.is_in(role_ids)).exec(db).await?;
    Ok(())
}

// api 格式 （api，method）
pub async fn update_api<C>(db: &C, old_api: (&str, &str), new_api: (&str, &str)) -> Result<UpdateResult>
where
    C: TransactionTrait + ConnectionTrait,
{
    let res = sys_role_api::Entity::update_many()
        .col_expr(sys_role_api::Column::Api, Expr::value(new_api.0))
        .col_expr(sys_role_api::Column::Method, Expr::value(new_api.1))
        .filter(sys_role_api::Column::Api.eq(old_api.0))
        .filter(sys_role_api::Column::Method.eq(old_api.1))
        .exec(db)
        .await
        .map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(res)
}
