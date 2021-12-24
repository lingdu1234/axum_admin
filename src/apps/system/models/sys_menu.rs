use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate, Clone)]
pub struct SearchReq {
    #[validate(length(min = 1))]
    pub id: Option<String>,
    #[validate(length(min = 1))]
    pub title: Option<String>,
    pub menu_type: Option<i8>,
    pub status: Option<i8>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Validate, FromQueryResult, Default)]
pub struct MenuResp {
    pub id: String,
    pub pid: String,
    pub route_path: String,
    pub title: String,
    pub icon: String,
    pub condition: String,
    pub remark: String,
    pub menu_type: i8,
    pub order_sort: i32,
    pub status: i8,
    pub always_show: i8,
    pub front_route_path: String,
    pub jump_path: String,
    pub component_path: String,
    pub allow_data_scope: i8,
    pub is_data_scope: i8,
    pub is_frame: i8,
    pub module_type: String,
    pub model_id: i32,
}

#[derive(Serialize, Clone, Validate, Debug, Default)]
pub struct UserMenu {
    #[serde(flatten)]
    pub menu: MenuResp,
    pub meta: Meta,
}

#[derive(Serialize, Clone, Validate, Debug, Default)]
pub struct Meta {
    pub icon: String,
    pub title: String,
    // keep_alive: bool,
}

#[derive(Serialize, Clone, Validate, Debug, Default)]
pub struct SysMenuTree {
    #[serde(flatten)]
    pub user_menu: UserMenu,
    pub children: Option<Vec<SysMenuTree>>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Validate)]
pub struct AddReq {
    pub pid: String,
    pub route_path: String,
    pub title: String,
    pub icon: Option<String>,
    pub condition: Option<String>,
    pub remark: Option<String>,
    pub menu_type: i8,
    pub order_sort: i32,
    pub status: i8,
    pub always_show: i8,
    pub front_route_path: Option<String>,
    pub jump_path: Option<String>,
    pub component_path: Option<String>,
    pub allow_data_scope: i8,
    pub is_data_scope: i8,
    pub is_frame: i8,
    pub module_type: String,
    pub model_id: i32,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct DeleteReq {
    pub menu_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct EditReq {
    pub id: String,
    pub pid: String,
    pub route_path: String,
    pub title: String,
    pub icon: String,
    pub condition: String,
    pub remark: String,
    pub menu_type: i8,
    pub order_sort: i32,
    pub status: i8,
    pub always_show: i8,
    pub front_route_path: String,
    pub jump_path: String,
    pub component_path: String,
    pub allow_data_scope: i8,
    pub is_data_scope: i8,
    pub is_frame: i8,
    pub module_type: String,
    pub model_id: i32,
}
