use std::fs::{self};

use db::system::entities::*;
pub use sea_orm::{ConnectionTrait, DatabaseConnection, DatabaseTransaction, Schema};
use sea_schema::migration::{
    sea_orm::{DatabaseBackend, Statement},
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

    db.execute(
        builder.build(
            schema
                .create_table_from_entity(sys_db_version::Entity)
                .to_owned()
                .if_not_exists(),
        ),
    )
    .await?;
    db.execute(
        builder.build(
            schema
                .create_table_from_entity(sys_user::Entity)
                .to_owned()
                .if_not_exists(),
        ),
    )
    .await?;
    db.execute(
        builder.build(
            schema
                .create_table_from_entity(sys_dept::Entity)
                .to_owned()
                .if_not_exists(),
        ),
    )
    .await?;
    db.execute(
        builder.build(
            schema
                .create_table_from_entity(sys_dict_type::Entity)
                .to_owned()
                .if_not_exists(),
        ),
    )
    .await?;
    db.execute(
        builder.build(
            schema
                .create_table_from_entity(sys_dict_data::Entity)
                .to_owned()
                .if_not_exists(),
        ),
    )
    .await?;
    db.execute(
        builder.build(
            schema
                .create_table_from_entity(sys_menu::Entity)
                .to_owned()
                .if_not_exists(),
        ),
    )
    .await?;
    db.execute(
        builder.build(
            schema
                .create_table_from_entity(sys_post::Entity)
                .to_owned()
                .if_not_exists(),
        ),
    )
    .await?;
    db.execute(
        builder.build(
            schema
                .create_table_from_entity(sys_user_post::Entity)
                .to_owned()
                .if_not_exists(),
        ),
    )
    .await?;
    db.execute(
        builder.build(
            schema
                .create_table_from_entity(sys_role::Entity)
                .to_owned()
                .if_not_exists(),
        ),
    )
    .await?;
    db.execute(
        builder.build(
            schema
                .create_table_from_entity(sys_role_dept::Entity)
                .to_owned()
                .if_not_exists(),
        ),
    )
    .await?;
    db.execute(
        builder.build(
            schema
                .create_table_from_entity(sys_login_log::Entity)
                .to_owned()
                .if_not_exists(),
        ),
    )
    .await?;
    db.execute(
        builder.build(
            schema
                .create_table_from_entity(sys_user_online::Entity)
                .to_owned()
                .if_not_exists(),
        ),
    )
    .await?;
    db.execute(
        builder.build(
            schema
                .create_table_from_entity(sys_job::Entity)
                .to_owned()
                .if_not_exists(),
        ),
    )
    .await?;
    db.execute(
        builder.build(
            schema
                .create_table_from_entity(sys_job_log::Entity)
                .to_owned()
                .if_not_exists(),
        ),
    )
    .await?;
    db.execute(
        builder.build(
            schema
                .create_table_from_entity(sys_oper_log::Entity)
                .to_owned()
                .if_not_exists(),
        ),
    )
    .await?;
    Ok(())
}

// 删除表格
async fn drop_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .drop_table(Table::drop().table(sys_db_version::Entity).to_owned())
        .await?;
    manager
        .drop_table(Table::drop().table(sys_user::Entity).to_owned())
        .await?;
    manager
        .drop_table(Table::drop().table(sys_dept::Entity).to_owned())
        .await?;
    manager
        .drop_table(Table::drop().table(sys_dict_type::Entity).to_owned())
        .await?;
    manager
        .drop_table(Table::drop().table(sys_dict_data::Entity).to_owned())
        .await?;
    manager
        .drop_table(Table::drop().table(sys_menu::Entity).to_owned())
        .await?;
    manager
        .drop_table(Table::drop().table(sys_post::Entity).to_owned())
        .await?;
    manager
        .drop_table(Table::drop().table(sys_user_post::Entity).to_owned())
        .await?;
    manager
        .drop_table(Table::drop().table(sys_role::Entity).to_owned())
        .await?;
    manager
        .drop_table(Table::drop().table(sys_role_dept::Entity).to_owned())
        .await?;
    manager
        .drop_table(Table::drop().table(sys_login_log::Entity).to_owned())
        .await?;
    manager
        .drop_table(Table::drop().table(sys_user_online::Entity).to_owned())
        .await?;
    manager
        .drop_table(Table::drop().table(sys_job::Entity).to_owned())
        .await?;
    manager
        .drop_table(Table::drop().table(sys_job_log::Entity).to_owned())
        .await?;
    manager
        .drop_table(Table::drop().table(sys_oper_log::Entity).to_owned())
        .await?;

    Ok(())
}

// 初始化数据
async fn init_data(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    let db = manager.get_connection();
    let builder = manager.get_database_backend();
    let dir = DATA_DIR.to_owned() + Migration.name();
    let rd = match fs::read_dir(&dir) {
        Ok(x) => x,
        Err(_) => {
            println!("{}", "数据文件不存在，请先确认迁移文件是否存在");
            return Ok(());
        }
    };
    for dir_entry in rd {
        let entry = dir_entry.expect("读取文件失败");
        // let fname = entry.file_name();
        let ori_str = fs::read_to_string(entry.path());
        let sql_string = match ori_str {
            Err(_) => {
                println!("{}", "读取文件失败");
                return Ok(());
            }
            Ok(x) => match builder {
                DatabaseBackend::Postgres => x.replace("`", ""),
                _ => x,
            },
        };

        let stmt = Statement::from_string(builder, sql_string).to_owned();
        db.execute(stmt).await?;
    }

    Ok(())
}
