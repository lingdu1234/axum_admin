use anyhow::{anyhow, Result};
use db::system::entities::*;
pub use sea_orm::{ConnectionTrait, DatabaseConnection, DatabaseTransaction, Schema};
use sea_schema::migration::{
    async_std::{
        fs::{self, File},
        io::{prelude::BufReadExt, BufReader},
        path::PathBuf,
        prelude::StreamExt,
    },
    sea_orm::{DatabaseBackend, EntityTrait, Statement},
    sea_query::*,
    *,
};

use crate::DATA_DIR;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        create_table(manager).await?;
        init_data(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        drop_table(manager).await
    }
}

/// 创建表格
async fn create_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    let db = manager.get_connection();
    // create_table(manager).await
    let builder = manager.get_database_backend();
    let schema = Schema::new(builder);

    creat_one_table(db, builder, &schema, sys_dept::Entity).await?;
    creat_one_table(db, builder, &schema, sys_dict_data::Entity).await?;
    creat_one_table(db, builder, &schema, sys_dict_type::Entity).await?;
    creat_one_table(db, builder, &schema, sys_job::Entity).await?;
    creat_one_table(db, builder, &schema, sys_menu::Entity).await?;
    creat_one_table(db, builder, &schema, sys_post::Entity).await?;
    creat_one_table(db, builder, &schema, sys_role_api::Entity).await?;
    creat_one_table(db, builder, &schema, sys_role_dept::Entity).await?;
    creat_one_table(db, builder, &schema, sys_role::Entity).await?;
    creat_one_table(db, builder, &schema, sys_user_post::Entity).await?;
    creat_one_table(db, builder, &schema, sys_user_role::Entity).await?;
    creat_one_table(db, builder, &schema, sys_user::Entity).await?;

    creat_one_table(db, builder, &schema, sys_user_online::Entity).await?;
    creat_one_table(db, builder, &schema, sys_job_log::Entity).await?;
    creat_one_table(db, builder, &schema, sys_oper_log::Entity).await?;
    creat_one_table(db, builder, &schema, sys_login_log::Entity).await?;

    Ok(())
}

// 删除表格
async fn drop_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    //
    drop_one_table(manager, sys_dept::Entity).await?;
    drop_one_table(manager, sys_dict_data::Entity).await?;
    drop_one_table(manager, sys_dict_type::Entity).await?;
    drop_one_table(manager, sys_job::Entity).await?;
    drop_one_table(manager, sys_menu::Entity).await?;
    drop_one_table(manager, sys_post::Entity).await?;
    drop_one_table(manager, sys_role_api::Entity).await?;
    drop_one_table(manager, sys_role_dept::Entity).await?;
    drop_one_table(manager, sys_role::Entity).await?;
    drop_one_table(manager, sys_user_post::Entity).await?;
    drop_one_table(manager, sys_user_role::Entity).await?;
    drop_one_table(manager, sys_user::Entity).await?;

    drop_one_table(manager, sys_user_online::Entity).await?;
    drop_one_table(manager, sys_job_log::Entity).await?;
    drop_one_table(manager, sys_oper_log::Entity).await?;
    drop_one_table(manager, sys_login_log::Entity).await?;

    Ok(())
}

// 初始化数据
async fn init_data(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    let db = manager.get_connection();
    let builder = manager.get_database_backend();
    let dir = DATA_DIR.to_owned() + Migration.name();
    let mut entries = match fs::read_dir(&dir).await {
        Ok(x) => x,
        Err(_) => {
            println!("{}", "数据文件不存在，请先确认迁移文件是否存在");
            return Ok(());
        }
    };
    while let Some(res) = entries.next().await {
        let entry = match res {
            Ok(v) => v,
            Err(e) => {
                println!("{}", e.to_string());
                return Ok(());
            }
        };
        let path = entry.path();
        let sql_vec = match get_insert_sql_string(path).await {
            Ok(v) => v,
            Err(e) => {
                println!("{:?}", e.to_string());
                return Ok(());
            }
        };
        for sql in sql_vec {
            let stmt = Statement::from_string(builder, sql).to_owned();
            db.execute(stmt).await?;
        }
    }

    Ok(())
}

async fn get_insert_sql_string(path: PathBuf) -> Result<Vec<String>> {
    let mut sql: Vec<String> = Vec::new();
    let mut sql_string = String::new();
    let file = match File::open(path).await {
        Ok(x) => x,
        Err(e) => return Err(anyhow!("读取文件失败:{:?}", e.to_string())),
    };
    let mut buf_reader = BufReader::new(file).lines();
    while let Some(line) = buf_reader.next().await {
        match line {
            Err(e) => return Err(anyhow!("读取行数据失败:{:?}", e.to_string())),
            Ok(v) => {
                if v.starts_with("/*!") || v.starts_with("--") {
                    continue;
                }
                let vv = if v.starts_with("INSERT") || v.starts_with("insert") {
                    v.replace("`", "")
                } else {
                    v
                };
                sql_string.push_str(&vv);
                if vv.ends_with(";") {
                    sql.push(sql_string.clone());
                    sql_string.clear();
                }
            }
        }
    }

    Ok(sql)
}

// 创建一张表
async fn creat_one_table<E>(
    db: &dyn ConnectionTrait,
    builder: DatabaseBackend,
    schema: &Schema,
    e: E,
) -> Result<(), DbErr>
where
    E: EntityTrait,
{
    db.execute(
        builder.build(
            schema
                .create_table_from_entity(e)
                .to_owned()
                .if_not_exists(),
        ),
    )
    .await?;
    Ok(())
}

// 删除一张表
async fn drop_one_table<T>(manager: &SchemaManager<'_>, t: T) -> Result<(), DbErr>
where
    T: IntoTableRef + 'static,
{
    manager
        .drop_table(Table::drop().table(t).to_owned())
        .await?;
    Ok(())
}
