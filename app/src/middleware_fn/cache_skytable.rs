use core::time::Duration;
use std::{collections::BTreeMap, sync::Arc, time::Instant};

use configs::CFG;
use db::common::{
    ctx::{ApiInfo, ReqCtx},
    res::ResJsonString,
};
use once_cell::sync::Lazy;
use poem::{Endpoint, IntoResponse, Middleware, Request, Response, Result};
use skytable::{
    actions::AsyncActions,
    pool::{AsyncPool, ConnectionManager},
};
use tokio::sync::{Mutex, OnceCell};

use crate::utils::{api_utils::ALL_APIS, jwt};

static SKY: OnceCell<AsyncPool> = OnceCell::const_new();

async fn set_sky() -> AsyncPool {
    let notls_manager = ConnectionManager::new_notls(&CFG.skytable.server, CFG.skytable.port);
    let notls_pool = AsyncPool::builder()
        .max_size(10)
        .build(notls_manager)
        .await
        .expect("skytable connect error please check it");
    //  第一次连接时，也就是程序启动时，清空数据
    notls_pool.get().await.expect("skytable connect error please check it").flushdb().await.unwrap();

    notls_pool
}
//  定义一个skytable 连接
pub async fn get_sky_table() -> &'static AsyncPool {
    let con = SKY.get_or_init(set_sky).await;
    con
}

//  程序中定义一个全局Map
// 用于存储已经缓存的数据，如果数据有更新,就清除该数据中对应的键值,
// 下次请求重新请求数据
pub static INDEX_MAP: Lazy<Arc<Mutex<BTreeMap<String, Instant>>>> = Lazy::new(|| {
    let data: BTreeMap<String, Instant> = BTreeMap::new();
    tokio::spawn(async { self::init().await });
    Arc::new(Mutex::new(data))
});
pub async fn init() {
    tracing::info!("cache data init");

    loop {
        tokio::time::sleep(Duration::from_secs(30)).await;
        init_loop().await;
    }
}

async fn init_loop() {
    let d = CFG.server.cache_time * 1000;
    let mut index = INDEX_MAP.lock().await;
    let mut data_keys: Vec<String> = Vec::new();
    for (k, v) in index.clone().iter() {
        if Instant::now().duration_since(*v).as_millis() as u64 > d {
            let key = k.split('★').collect::<Vec<&str>>();
            data_keys.push(key[1].to_string());
            // 移除缓存索引
            index.remove(k);
        }
    }
    drop(index);
    if data_keys.len() != 0 {
        // 移除缓存数据
        remove_cache_data(data_keys).await;
    }
}
//  添加索引
async fn add_index_map(api_key: &str, data_key: &str) {
    let mut index = INDEX_MAP.lock().await;
    let key = get_key(api_key, data_key);
    index.insert(key, Instant::now());
    drop(index);
}
//  删除索引
async fn remove_index_map(api_keys: Option<Vec<String>>) {
    let mut index = INDEX_MAP.lock().await;
    let mut data_keys: Vec<String> = Vec::new();
    if api_keys.is_some() {
        for api_key in api_keys.unwrap() {
            for (k, _) in index.clone().into_iter() {
                if k.starts_with(&api_key) {
                    index.remove(&k);
                    let key = k.split('★').collect::<Vec<&str>>();
                    data_keys.push(key[1].to_string())
                }
            }
        }
    }
    // 在这里删除数据
    remove_cache_data(data_keys).await;
    drop(index);
}
//  获取索引是否存在
async fn get_index_map(api_key: &str, data_key: &str) -> bool {
    let key = get_key(api_key, data_key);
    let index = INDEX_MAP.lock().await;
    let res = index.get(&key).is_some();
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

    add_index_map(api_key, data_key).await;

    match con.get().await.unwrap().get::<String>(data_key).await {
        Ok(_) => match con.get().await.unwrap().update(data_key, data).await {
            Ok(_) => tracing::info!("update cache data OK,api_key: {}, data_key: {},api:{}", api_key, data_key, ori_uri),
            Err(_) => tracing::info!("update cache data error,api_key: {}, data_key: {},api:{}", api_key, data_key, ori_uri),
        },
        Err(_) => match con.get().await.unwrap().set(data_key, data).await {
            Ok(_) => tracing::info!("add cache data OK,api_key: {}, data_key: {},api:{}", api_key, data_key, ori_uri),
            Err(_) => tracing::info!("add cache data error,api_key: {}, data_key: {},api:{}", api_key, data_key, ori_uri),
        },
    };
}

//  获取数据
pub async fn get_cache_data(api_key: &str, data_key: &str) -> Option<String> {
    let con = get_sky_table().await;

    match get_index_map(api_key, data_key).await {
        false => None,
        true => {
            let data: Option<String> = match con.get().await.unwrap().get::<String>(data_key).await {
                Ok(v) => Some(v),
                Err(_) => None,
            };
            tracing::info!("get cache data success,api_key: {}, data_key: {}", api_key, data_key);
            data
        }
    }
}

//  移除数据
pub async fn remove_cache_data(data_keys: Vec<String>) {
    let con = get_sky_table().await;
    match con.get().await.unwrap().del(&data_keys).await {
        Ok(v) => tracing::info!("remove cache data success,data_keys: {:?},total:{}", &data_keys, v),
        Err(e) => tracing::info!("remove cache data failed,data_keys: {:?},error:{}", &data_keys, e),
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
