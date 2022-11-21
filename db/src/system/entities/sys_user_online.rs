//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "sys_user_online"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize,ToSchema)]
pub struct Model {
    pub id: String,
    pub u_id: String,
    pub token_id: String,
    pub token_exp: i64,
    pub login_time: DateTime,
    pub user_name: String,
    pub dept_name: String,
    pub net: String,
    pub ipaddr: String,
    pub login_location: String,
    pub device: String,
    pub browser: String,
    pub os: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    UId,
    TokenId,
    TokenExp,
    LoginTime,
    UserName,
    DeptName,
    Net,
    Ipaddr,
    LoginLocation,
    Device,
    Browser,
    Os,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = String;
    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::String(Some(32u32)).def(),
            Self::UId => ColumnType::String(Some(32u32)).def(),
            Self::TokenId => ColumnType::String(Some(32u32)).def(),
            Self::TokenExp => ColumnType::BigInteger.def(),
            Self::LoginTime => ColumnType::DateTime.def(),
            Self::UserName => ColumnType::String(Some(255u32)).def(),
            Self::DeptName => ColumnType::String(Some(100u32)).def(),
            Self::Net => ColumnType::String(Some(10u32)).def(),
            Self::Ipaddr => ColumnType::String(Some(120u32)).def(),
            Self::LoginLocation => ColumnType::String(Some(255u32)).def(),
            Self::Device => ColumnType::String(Some(50u32)).def(),
            Self::Browser => ColumnType::String(Some(30u32)).def(),
            Self::Os => ColumnType::String(Some(30u32)).def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
