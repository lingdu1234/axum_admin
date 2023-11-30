use anyhow::Result;
pub use sea_orm::{ConnectionTrait, DatabaseConnection, DatabaseTransaction, Schema};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230403_000001_add_i18n"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        println!("Executing: ALTER TABLE `sys_menu` ADD COLUMN `i18n` varchar(200)");

        manager
            .alter_table(
                sea_query::Table::alter()
                    .table(Alias::new("sys_menu"))
                    .add_column(ColumnDef::new(Alias::new("i18n")).string_len(200))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        println!("Executing: ALTER TABLE `sys_menu` DROP COLUMN `i18n`;");

        manager
            .alter_table(Table::alter().table(Alias::new("sys_menu")).drop_column(Alias::new("i18n")).to_owned())
            .await?;

        Ok(())
    }
}

