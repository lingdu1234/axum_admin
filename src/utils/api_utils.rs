use std::{collections::HashMap, sync::Arc};

use db::{db_conn, system::entities::sys_role_api, DB};
use once_cell::sync::Lazy;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tokio::sync::Mutex;
use tracing::info;

use crate::apps::system;

pub static ALL_APIS: Lazy<Arc<Mutex<HashMap<String, String>>>> = Lazy::new(|| {
    let apis: HashMap<String, String> = HashMap::new();
    Arc::new(Mutex::new(apis))
});

pub async fn init_all_api() {
    let db = DB.get_or_init(db_conn).await;
    let res = system::get_all_sys_menu(db).await;
    let mut apis = ALL_APIS.lock().await;
    match res {
        Ok(menus) => {
            for menu in menus {
                apis.insert(menu.api.clone(), menu.menu_name.clone());
            }
        }
        Err(e) => {
            info!("初始化时获取路由API失败:{:#?}", e)
        }
    }
}

pub async fn add_api(api: &str, menu_name: &str) {
    let mut apis = ALL_APIS.lock().await;
    apis.insert(api.to_string(), menu_name.to_string());
}

pub async fn remove_api(api: &str) {
    let mut apis = ALL_APIS.lock().await;
    apis.remove(api);
}

pub async fn is_in(api: &str) -> bool {
    let apis = ALL_APIS.lock().await;
    apis.get(api).is_some()
}

pub async fn check_api_permission(api: &str, method: &str) -> bool {
    let db = DB.get_or_init(db_conn).await;
    match sys_role_api::Entity::find()
        .filter(sys_role_api::Column::Api.eq(api))
        .filter(sys_role_api::Column::Method.eq(method))
        .one(db)
        .await
    {
        Ok(x) => x.is_some(),
        Err(e) => {
            info!("检查API权限出现错误:{:#?}", e);
            false
        }
    }
}

// pub async fn check_api_permission(api: &str, method: &str) -> bool {
//     let e = super::get_enforcer(false).await;
//     match e.enforce((api, method)) {
//         Ok(_) => true,
//         Err(err) => {
//             info!("检查权限失败:{:#?}", err);
//             false
//         }
//     }
// }
