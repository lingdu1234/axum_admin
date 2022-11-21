use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

use crate::system::entities::sys_menu;

#[derive(Deserialize, Clone)]
pub struct SysMenuSearchReq {
    pub id: Option<String>,
    pub menu_name: Option<String>,
    pub menu_type: Option<String>,
    pub menu_types: Option<String>,
    pub method: Option<String>,
    pub status: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct SysMenuTreeAll {
    #[serde(flatten)]
    pub menu: sys_menu::Model,
    pub children: Option<Vec<SysMenuTreeAll>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, FromQueryResult, Default)]
pub struct MenuResp {
    pub id: String,
    pub pid: String,
    pub path: String,
    pub menu_name: String,
    pub icon: String,
    pub menu_type: String,
    pub query: Option<String>,
    pub order_sort: i32,
    pub status: String,
    pub api: String,
    pub method: String,
    pub component: String,
    pub visible: String,
    pub is_frame: String,
    pub is_cache: String,
    pub data_scope: String,
    pub log_method: String,
    pub i18n: Option<String>,
    pub data_cache_method: String,
    pub remark: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct MenuRelated {
    #[serde(flatten)]
    pub menu: sys_menu::Model,
    pub dbs: Vec<String>,
    pub apis: Vec<String>,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct UserMenu {
    pub id: String,
    pub pid: String,
    pub always_show: Option<bool>,
    pub path: String,
    pub name: String,
    pub menu_name: String,
    pub menu_type: String,
    pub component: String,
    pub hidden: bool,
    pub meta: Meta,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct Meta {
    pub icon: String,
    pub title: String,
    pub link: Option<String>,
    pub no_cache: bool,
    pub hidden: bool,
    pub i18n: Option<String>,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct SysMenuTree {
    #[serde(flatten)]
    pub user_menu: UserMenu,
    pub children: Option<Vec<SysMenuTree>>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SysMenuAddReq {
    pub pid: String,
    pub path: Option<String>,
    pub menu_name: String,
    pub icon: Option<String>,
    pub menu_type: String,
    pub query: Option<String>,
    pub order_sort: i32,
    pub status: String,
    pub api: String,
    pub method: Option<String>,
    pub component: Option<String>,
    pub visible: String,
    pub is_frame: String,
    pub is_cache: String,
    pub data_scope: String,
    pub log_method: String,
    pub data_cache_method: String,
    pub i18n: Option<String>,
    pub remark: String,
}

#[derive(Debug, Deserialize)]
pub struct SysMenuDeleteReq {
    pub id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SysMenuEditReq {
    pub id: String,
    pub pid: String,
    pub path: String,
    pub menu_name: String,
    pub icon: Option<String>,
    pub menu_type: String,
    pub query: Option<String>,
    pub order_sort: i32,
    pub status: String,
    pub api: String,
    pub method: Option<String>,
    pub component: String,
    pub visible: String,
    pub is_frame: String,
    pub is_cache: String,
    pub data_scope: String,
    pub log_method: String,
    pub i18n: Option<String>,
    pub data_cache_method: String,
    pub remark: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogCacheEditReq {
    pub id: String,
    pub log_method: String,
    pub data_cache_method: String,
}
