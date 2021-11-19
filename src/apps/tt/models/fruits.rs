//! SeaORM Entity. Generated by sea-orm-codegen 0.3.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "fruits")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: i32,
    pub name: String,
    pub count: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            _ => panic!("No RelationDef"),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}