#![allow(non_camel_case_types)]

pub use sea_orm_migration::prelude::*;

pub mod common;
pub mod db_utils;
mod migrations;

pub use migrations::*;

include!(concat!(env!("OUT_DIR"), "/auto_migrations.rs"));

pub struct Migrator;
pub static DATA_DIR: &str = "migration/data/";

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        all_migrations()
    }
}
