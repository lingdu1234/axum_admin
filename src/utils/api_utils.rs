use std::{collections::HashMap, sync::Arc};

use db::{common::ctx::ApiInfo, db_conn, system::entities::sys_role_api, DB};
use once_cell::sync::Lazy;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, TransactionTrait};
use tokio::sync::Mutex;
use tracing::info;

use crate::apps::system;

pub static ALL_APIS: Lazy<Arc<Mutex<HashMap<String, ApiInfo>>>> = Lazy::new(|| {
    let apis: HashMap<String, ApiInfo> = HashMap::new();
    Arc::new(Mutex::new(apis))
});

pub async fn init_all_api() {
    let db = DB.get_or_init(db_conn).await;
    let res = system::get_all_sys_menu(db, false).await;
    match res {
        Ok(menus) => {
            for menu in menus {
                self::add_api(db, &menu.id, &menu.api, &menu.menu_name, &menu.is_db_cache, &menu.is_log).await;
            }
            let apis = ALL_APIS.lock().await;
            info!("初始化时获取路由API成功:{:#?}", apis);
        }
        Err(e) => {
            info!("初始化时获取路由API失败:{:#?}", e)
        }
    }
}

pub async fn add_api<C>(db: &C, api_id: &str, api: &str, menu_name: &str, is_db_cache: &str, is_log: &str)
where
    C: TransactionTrait + ConnectionTrait,
{
    let mut apis = ALL_APIS.lock().await;
    let related_api = match system::get_related_api_by_db_name(db, api_id).await {
        Ok(x) => Some(x),
        Err(e) => {
            info!("{}", e);
            None
        }
    };
    apis.insert(
        api.to_string(),
        ApiInfo {
            name: menu_name.to_string(),
            related_api,
            is_db_cache: is_db_cache == "1",
            is_log: is_log == "1",
        },
    );
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
