use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use crate::utils::{api_utils::ALL_APIS, jwt};
use configs::CFG;
use db::common::{
    ctx::{ApiInfo, ReqCtx},
    res::ResJsonString,
};
use once_cell::sync::Lazy;
use poem::{Endpoint, IntoResponse, Middleware, Request, Response, Result};
use skytable::actions::AsyncActions;
use skytable::pool::{AsyncPool, ConnectionManager};
use tokio::sync::{Mutex, OnceCell};

static SKY: OnceCell<AsyncPool> = OnceCell::const_new();

async fn set_sky() -> AsyncPool {
    let notls_manager = ConnectionManager::new_notls(&CFG.skytable.server, CFG.skytable.port);
    let notls_pool = AsyncPool::builder().max_size(10).build(notls_manager).await.unwrap();
    //  第一次连接时，也就是程序启动时，清空数据
    notls_pool.get().await.unwrap().flushdb().await.unwrap();

    notls_pool
}
//  定义一个skytable 连接
pub async fn get_sky_table() -> &'static AsyncPool {
    let con = SKY.get_or_init(set_sky).await;
    con
}

//  程序中定义一个全局hashSet 用于存储已经缓存的数据，如果数据有更新,就清除该数据中对应的键值,下次请求重新请求数据
static INDEX_MAP: Lazy<Arc<Mutex<HashMap<String, HashSet<String>>>>> = Lazy::new(|| {
    let data: HashMap<String, HashSet<String>> = HashMap::new();
    Arc::new(Mutex::new(data))
});
//  添加索引
async fn add_index_map(api_key: &str, data_key: &str) {
    let mut index = INDEX_MAP.lock().await;
    let v = index.entry(api_key.to_string()).or_insert(HashSet::new());
    v.insert(data_key.to_string());
    drop(index);
}
//  删除索引
async fn remove_index_map(api_keys: Option<Vec<String>>) {
    let mut index = INDEX_MAP.lock().await;

    if api_keys.is_some() {
        for api_key in api_keys.unwrap() {
            index.remove(&api_key);
        }
    }
    drop(index);
}
//  获取索引是否存在
async fn get_index_map(api_key: &str, data_key: &str) -> bool {
    let mut index = INDEX_MAP.lock().await;
    let v = index.entry(api_key.to_string()).or_insert(HashSet::new());
    let res = v.get(data_key).is_some();
    drop(index);
    res
}

//  获取key
fn get_key(api_key: &str, data_key: &str) -> String {
    format!("{}★{}", api_key, &data_key)
}

//  添加数据
pub async fn add_cache_data(ori_uri: &str, api_key: &str, data_key: &str, data: String) {
    let con = get_sky_table().await;
    let key = get_key(api_key, data_key);

    add_index_map(api_key, data_key).await;

    match con.get().await.unwrap().get::<String>(&key).await {
        Ok(_) => match con.get().await.unwrap().update(&key, data).await {
            Ok(_) => tracing::info!("update cache data OK,api_key: {}, data_key: {},api:{}", api_key, data_key, ori_uri),
            Err(_) => tracing::info!("update cache data error,api_key: {}, data_key: {},api:{}", api_key, data_key, ori_uri),
        },
        Err(_) => match con.get().await.unwrap().set(&key, data).await {
            Ok(_) => tracing::info!("add cache data OK,api_key: {}, data_key: {},api:{}", api_key, data_key, ori_uri),
            Err(_) => tracing::info!("add cache data error,api_key: {}, data_key: {},api:{}", api_key, data_key, ori_uri),
        },
    };
}

//  获取数据
pub async fn get_cache_data(api_key: &str, data_key: &str) -> Option<String> {
    let con = get_sky_table().await;
    let key = get_key(api_key, data_key);

    match get_index_map(api_key, data_key).await {
        false => None,
        true => {
            let data: Option<String> = match con.get().await.unwrap().get::<String>(key).await {
                Ok(v) => Some(v),
                Err(_) => None,
            };
            tracing::info!("get cache data success,api_key: {}, data_key: {}", api_key, data_key);
            data
        }
    }
}

pub struct SkyTableCache;

impl<E: Endpoint> Middleware<E> for SkyTableCache {
    type Output = CacheEndpoint<E>;

    fn transform(&self, ep: E) -> Self::Output {
        CacheEndpoint { ep }
    }
}

/// Endpoint for `Tracing` middleware.
pub struct CacheEndpoint<E> {
    ep: E,
}

#[poem::async_trait]
impl<E: Endpoint> Endpoint for CacheEndpoint<E> {
    // type Output = E::Output;
    type Output = Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        let apis = ALL_APIS.lock().await;
        let ctx = req.extensions().get::<ReqCtx>().expect("ReqCtx not found").clone();

        let api_info = match apis.get(&ctx.path) {
            Some(x) => x.clone(),
            None => ApiInfo {
                name: "".to_string(),
                data_cache_method: "0".to_string(),
                log_method: "0".to_string(),
                related_api: None,
            },
        };
        // 释放锁
        drop(apis);
        let (token_id, _) = jwt::get_bear_token(&req).await?;

        if ctx.method.as_str() != "GET" {
            let res_end = self.ep.call(req).await;
            return match res_end {
                Ok(v) => {
                    let related_api = api_info.related_api.clone();
                    tokio::spawn(async move {
                        remove_index_map(related_api).await;
                    });
                    Ok(v.into_response())
                }
                Err(e) => Err(e),
            };
        }
        let data_key = match api_info.data_cache_method.clone().as_str() {
            "1" => format!("{}_{}_{}", &ctx.ori_uri, &ctx.method, &token_id),
            _ => format!("{}_{}", &ctx.ori_uri, &ctx.method),
        };
        // 开始请求数据
        match api_info.data_cache_method.as_str() {
            "0" => {
                let res_end = self.ep.call(req).await;
                match res_end {
                    Ok(v) => Ok(v.into_response()),
                    Err(e) => Err(e),
                }
            }
            _ => match get_cache_data(&ctx.path, &data_key).await {
                Some(v) => Ok(v.into_response()),
                None => {
                    let res_end = self.ep.call(req).await;
                    match res_end {
                        Ok(v) => {
                            let res = v.into_response();
                            let res_ctx = match res.extensions().get::<ResJsonString>() {
                                Some(x) => x.0.clone(),
                                None => "".to_string(),
                            };

                            tokio::spawn(async move {
                                // 缓存数据
                                add_cache_data(&ctx.ori_uri, &ctx.path, &data_key, res_ctx).await;
                            });

                            Ok(res)
                        }
                        Err(e) => Err(e),
                    }
                }
            },
        }
    }
}

// 感觉没有什么鸟用
