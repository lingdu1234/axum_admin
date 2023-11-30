pub use sea_orm_migration::prelude::*;

pub mod db_utils;
mod migrations;

pub use migrations::*;

pub struct Migrator;
pub static DATA_DIR: &str = "migration/data/";

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000001_create_table::Migration), Box::new(m20230403_000001_add_i18n::Migration)]
    }
}
