use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct SysRoleSearchReq {
    pub role_id: Option<String>,
    pub role_ids: Option<String>,
    pub role_name: Option<String>,
    pub role_key: Option<String>,
    pub status: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SysRoleAddReq {
    pub role_name: String,
    pub role_key: String,
    pub list_order: i32,
    pub data_scope: Option<String>,
    pub status: String,
    pub remark: Option<String>,
    pub menu_ids: Vec<String>,
}

#[derive(Deserialize)]
pub struct SysRoleDeleteReq {
    pub role_ids: Vec<String>,
}
#[derive(Deserialize)]
pub struct DataScopeReq {
    pub role_id: String,
    pub data_scope: String,
    pub dept_ids: Vec<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SysRoleEditReq {
    pub role_id: String,
    pub role_name: String,
    pub role_key: String,
    pub list_order: i32,
    pub data_scope: String,
    pub status: String,
    pub remark: Option<String>,
    pub menu_ids: Vec<String>,
}
#[derive(Deserialize, Clone)]
pub struct SysRoleStatusReq {
    pub role_id: String,
    pub status: String,
}
#[derive(Deserialize, Clone)]
pub struct UpdateAuthRoleReq {
    pub user_id: String,
    pub role_ids: Vec<String>,
}

#[derive(Deserialize, Clone)]
pub struct AddOrCancelAuthRoleReq {
    pub user_ids: Vec<String>,
    pub role_id: String,
}

#[derive(Debug, Serialize, FromQueryResult, Clone)]
pub struct SysRoleResp {
    pub role_id: String,
    pub role_name: String,
    pub role_key: String,
    pub status: String,
    pub list_order: i32,
    pub remark: String,
    pub data_scope: String,
}
