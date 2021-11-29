use crate::models::{ActiveModel, Column, Entity, Model, NewCasbinRule};
use crate::Error;
use casbin::{error::AdapterError, Error as CasbinError, Filter, Result};
use sea_orm::sea_query::{self, ColumnDef, Condition};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, ExecResult,
    QueryFilter, Set,
};
pub async fn new(conn: &DatabaseConnection) -> Result<ExecResult> {
    let stmt = sea_query::Table::create()
        .table(Entity)
        .if_not_exists()
        .col(ColumnDef::new(Column::Id).string().not_null().primary_key())
        .col(ColumnDef::new(Column::Ptype).string())
        .col(ColumnDef::new(Column::V0).string())
        .col(ColumnDef::new(Column::V1).string())
        .col(ColumnDef::new(Column::V2).string())
        .col(ColumnDef::new(Column::V3).string())
        .col(ColumnDef::new(Column::V4).string())
        .col(ColumnDef::new(Column::V5).string())
        .to_owned();
    // 创建表格
    let builder = conn.get_database_backend();
    conn.execute(builder.build(&stmt))
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))
    // create_table(conn, &stmt).await?;
}

///  删除策略
pub async fn remove_policy(conn: &DatabaseConnection, pt: &str, rule: Vec<String>) -> Result<bool> {
    let rule = normalize_casbin_rule(rule, 0);
    let d: ActiveModel = Entity::find()
        .filter(Column::Ptype.eq(pt))
        .filter(Column::V0.eq((&rule[0]).clone()))
        .filter(Column::V1.eq((&rule[1]).clone()))
        .filter(Column::V2.eq((&rule[2]).clone()))
        .filter(Column::V3.eq((&rule[3]).clone()))
        .filter(Column::V4.eq((&rule[4]).clone()))
        .filter(Column::V5.eq((&rule[5]).clone()))
        .one(conn)
        .await
        .unwrap()
        .unwrap()
        .into();
    d.delete(conn)
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))?;
    Ok(true)
}

///  删除多个策略
pub async fn remove_policies(
    conn: &DatabaseConnection,
    pt: &str,
    rules: Vec<Vec<String>>,
) -> Result<bool> {
    for rule in rules {
        let rule = normalize_casbin_rule(rule, 0);
        let d: ActiveModel = Entity::find()
            .filter(Column::Ptype.eq(pt))
            .filter(Column::V0.eq((&rule[0]).clone()))
            .filter(Column::V1.eq((&rule[1]).clone()))
            .filter(Column::V2.eq((&rule[2]).clone()))
            .filter(Column::V3.eq((&rule[3]).clone()))
            .filter(Column::V4.eq((&rule[4]).clone()))
            .filter(Column::V5.eq((&rule[5]).clone()))
            .one(conn)
            .await
            .unwrap()
            .unwrap()
            .into();
        d.delete(conn)
            .await
            .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))?;
    }

    Ok(true)
}

///  删除筛选的策略
pub async fn remove_filtered_policy(
    conn: &DatabaseConnection,
    pt: &str,
    field_index: usize,
    field_values: Vec<String>,
) -> Result<bool> {
    let field_values = normalize_casbin_rule(field_values, field_index);
    let for_delete: ActiveModel = if field_index == 5 {
        Entity::find()
            .filter(
                Condition::all()
                    .add(Column::Ptype.eq(pt))
                    .add(Column::V5.eq((field_values[5]).clone())),
            )
            .one(conn)
            .await
            .unwrap()
            .unwrap()
            .into()
    } else if field_index == 4 {
        Entity::find()
            .filter(
                Condition::all()
                    .add(Column::Ptype.eq(pt))
                    .add(Column::V4.eq((field_values[4]).clone()))
                    .add(Column::V5.eq((field_values[5]).clone())),
            )
            .one(conn)
            .await
            .unwrap()
            .unwrap()
            .into()
    } else if field_index == 3 {
        Entity::find()
            .filter(
                Condition::all()
                    .add(Column::Ptype.eq(pt))
                    .add(Column::V3.eq((field_values[3]).clone()))
                    .add(Column::V4.eq((field_values[4]).clone()))
                    .add(Column::V5.eq((field_values[5]).clone())),
            )
            .one(conn)
            .await
            .unwrap()
            .unwrap()
            .into()
    } else if field_index == 2 {
        Entity::find()
            .filter(
                Condition::all()
                    .add(Column::Ptype.eq(pt))
                    .add(Column::V2.eq((field_values[2]).clone()))
                    .add(Column::V3.eq((field_values[3]).clone()))
                    .add(Column::V4.eq((field_values[4]).clone()))
                    .add(Column::V5.eq((field_values[5]).clone())),
            )
            .one(conn)
            .await
            .unwrap()
            .unwrap()
            .into()
    } else if field_index == 1 {
        Entity::find()
            .filter(
                Condition::all()
                    .add(Column::Ptype.eq(pt))
                    .add(Column::V1.eq((field_values[1]).clone()))
                    .add(Column::V2.eq((field_values[2]).clone()))
                    .add(Column::V3.eq((field_values[3]).clone()))
                    .add(Column::V4.eq((field_values[4]).clone()))
                    .add(Column::V5.eq((field_values[5]).clone())),
            )
            .one(conn)
            .await
            .unwrap()
            .unwrap()
            .into()
    } else {
        Entity::find()
            .filter(
                Condition::all()
                    .add(Column::Ptype.eq(pt))
                    .add(Column::V0.eq((field_values[0]).clone()))
                    .add(Column::V1.eq((field_values[1]).clone()))
                    .add(Column::V2.eq((field_values[2]).clone()))
                    .add(Column::V3.eq((field_values[3]).clone()))
                    .add(Column::V4.eq((field_values[4]).clone()))
                    .add(Column::V5.eq((field_values[5]).clone())),
            )
            .one(conn)
            .await
            .unwrap()
            .unwrap()
            .into()
    };
    for_delete
        .delete(conn)
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))?;

    Ok(true)
}
//  加载策略
pub async fn load_policy(conn: &DatabaseConnection) -> Result<Vec<Model>> {
    let casbin_rule = Entity::find()
        .all(conn)
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))?;

    Ok(casbin_rule)
}

pub(crate) async fn load_filtered_policy<'a>(
    conn: &DatabaseConnection,
    filter_x: &Filter<'_>,
) -> Result<Vec<Model>> {
    let (g_filter, p_filter) = filtered_where_values(filter_x);
    let casbin_rule = Entity::find()
        .filter(
            Condition::any()
                .add(
                    Condition::all()
                        .add(Column::Ptype.like("p%"))
                        .add(Column::V0.like((g_filter[0]).clone()))
                        .add(Column::V1.like((g_filter[1]).clone()))
                        .add(Column::V2.like((g_filter[2]).clone()))
                        .add(Column::V3.like((g_filter[3]).clone()))
                        .add(Column::V4.like((g_filter[4]).clone()))
                        .add(Column::V5.like((g_filter[5]).clone())),
                )
                .add(
                    Condition::all()
                        .add(Column::Ptype.like("g%"))
                        .add(Column::V0.like((p_filter[0]).clone()))
                        .add(Column::V1.like((p_filter[1]).clone()))
                        .add(Column::V2.like((p_filter[2]).clone()))
                        .add(Column::V3.like((p_filter[3]).clone()))
                        .add(Column::V4.like((p_filter[4]).clone()))
                        .add(Column::V5.like((p_filter[5]).clone())),
                ),
        )
        .all(conn)
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))?;
    Ok(casbin_rule)
}

///  保存策略
pub async fn save_policy(conn: &DatabaseConnection, rules: Vec<NewCasbinRule>) -> Result<()> {
    let mut txn = conn
        .begin()
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))?;
    for rule in rules {
        let uid = scru128::scru128();
        let p = ActiveModel {
            id: Set(uid),
            ptype: Set(rule.ptype),
            v0: Set(rule.v0),
            v1: Set(rule.v1.unwrap_or("".to_string())),
            v2: Set(rule.v2.unwrap_or("".to_string())),
            v3: Set(rule.v3.unwrap_or("".to_string())),
            v4: Set(rule.v4.unwrap_or("".to_string())),
            v5: Set(rule.v5.unwrap_or("".to_string())),
        };
        p.insert(&mut txn)
            .await
            .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))?;
    }

    txn.commit()
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))?;
    Ok(())
}

///  添加策略
pub(crate) async fn add_policy(conn: &DatabaseConnection, rule: NewCasbinRule) -> Result<bool> {
    let uid = scru128::scru128();
    let p = ActiveModel {
        id: Set(uid),
        ptype: Set(rule.ptype),
        v0: Set(rule.v0),
        v1: Set(rule.v1.unwrap_or("".to_string())),
        v2: Set(rule.v2.unwrap_or("".to_string())),
        v3: Set(rule.v3.unwrap_or("".to_string())),
        v4: Set(rule.v4.unwrap_or("".to_string())),
        v5: Set(rule.v5.unwrap_or("".to_string())),
    };
    p.insert(conn)
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))?;
    Ok(true)
}

///  添加多个策略
pub(crate) async fn add_policies(
    conn: &DatabaseConnection,
    rules: Vec<NewCasbinRule>,
) -> Result<bool> {
    let mut txn = conn
        .begin()
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))?;
    for rule in rules {
        let uid = scru128::scru128();
        let p = ActiveModel {
            id: Set(uid),
            ptype: Set(rule.ptype),
            v0: Set(rule.v0),
            v1: Set(rule.v1.unwrap_or("".to_string())),
            v2: Set(rule.v2.unwrap_or("".to_string())),
            v3: Set(rule.v3.unwrap_or("".to_string())),
            v4: Set(rule.v4.unwrap_or("".to_string())),
            v5: Set(rule.v5.unwrap_or("".to_string())),
        };
        p.insert(&mut txn)
            .await
            .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))?;
    }
    txn.commit()
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))?;
    Ok(true)
}
///  清除策略
pub(crate) async fn clear_policy(conn: &DatabaseConnection) -> Result<()> {
    let mut txn = conn
        .begin()
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))?;
    Entity::delete_many()
        .exec(&mut txn)
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))?;
    txn.commit()
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))?;
    Ok(())
}

///  
///
///  将策略数组格式化成全部位置都有数据
fn normalize_casbin_rule(mut rule: Vec<String>, field_index: usize) -> Vec<String> {
    rule.resize(6 - field_index, String::from(""));
    rule
}
/// 获取筛选参数
fn filtered_where_values<'a>(filter: &Filter<'a>) -> ([&'a str; 6], [&'a str; 6]) {
    let mut g_filter: [&'a str; 6] = ["%", "%", "%", "%", "%", "%"];
    let mut p_filter: [&'a str; 6] = ["%", "%", "%", "%", "%", "%"];
    for (idx, val) in filter.g.iter().enumerate() {
        if val != &"" {
            g_filter[idx] = val;
        }
    }
    for (idx, val) in filter.p.iter().enumerate() {
        if val != &"" {
            p_filter[idx] = val;
        }
    }
    (g_filter, p_filter)
}
