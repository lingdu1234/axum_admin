use core::time::Duration;
use std::{collections::BTreeMap, sync::Arc, time::Instant};

use ahash::AHashMap as HashMap;
use configs::CFG;
use db::common::{
    ctx::{ApiInfo, ReqCtx},
    res::ResJsonString,
};
use once_cell::sync::Lazy;
use poem::{Endpoint, IntoResponse, Middleware, Request, Response, Result};
use tokio::sync::Mutex;

use crate::utils::{api_utils::ALL_APIS, jwt};

#[allow(clippy::type_complexity)]
pub static RES_DATA: Lazy<Arc<Mutex<HashMap<String, HashMap<String, String>>>>> = Lazy::new(|| {
    let data: HashMap<String, HashMap<String, String>> = HashMap::new();
    Arc::new(Mutex::new(data))
});

// 格式 token★apipath
pub static RES_INDEX: Lazy<Arc<Mutex<BTreeMap<String, Instant>>>> = Lazy::new(|| {
    let inddex: BTreeMap<String, Instant> = BTreeMap::new();
    tokio::spawn(async { self::init().await });
    Arc::new(Mutex::new(inddex))
});

pub async fn init() {
    tracing::info!("cache data init");

    loop {
        tokio::time::sleep(Duration::from_secs(300)).await;
        init_loop().await;
    }
}

async fn init_loop() {
    let d = CFG.server.cache_time * 1000;
    let mut res_bmap = RES_INDEX.lock().await;
    for (k, v) in res_bmap.clone().iter() {
        if Instant::now().duration_since(*v).as_millis() as u64 > d {
            // ★ 前为api，后面为 data_key
            let key = k.split('★').collect::<Vec<&str>>();
            remove_cache_data(key[0], None, Some(key[1])).await;
            res_bmap.remove(k);
        } else {
            break;
        }
    }
}

pub async fn add_cache_data(ori_uri: &str, api_key: &str, data_key: &str, data: String) {
    let mut res_bmap = RES_INDEX.lock().await;
    let index_key = format!("{}★{}", api_key, &data_key);
    res_bmap.insert(index_key.clone(), Instant::now());
    drop(res_bmap);
    let hmap: HashMap<String, String> = HashMap::new();
    let mut res_data = RES_DATA.lock().await;
    let v = res_data.entry(api_key.to_string()).or_insert(hmap);
    v.insert(data_key.to_string(), data);
    drop(res_data);
    tracing::info!("add cache data,api_key: {}, data_key: {},api:{}", api_key, data_key, ori_uri);
}

pub async fn get_cache_data(api_key: &str, data_key: &str) -> Option<String> {
    let res_data = RES_DATA.lock().await;

    let h = match res_data.get(api_key) {
        Some(v) => v,
        None => return None,
    };
    let res = match h.get(data_key) {
        Some(v) => Some(v.clone()),
        None => return None,
    };
    drop(res_data);
    tracing::info!("get cache data success,api_key: {}, data_key: {}", api_key, data_key);
    res
}

pub async fn remove_cache_data(api_key: &str, related_api: Option<Vec<String>>, data_key: Option<&str>) {
    let mut res_data = RES_DATA.lock().await;

    match data_key {
        None => {
            //  获取影响的所有key
            match related_api {
                Some(apis) => {
                    for api in &apis {
                        res_data.remove(api);
                    }
                    tracing::info!("remove cache data: apis:{:?}", apis);
                }
                None => {
                    res_data.remove(api_key);
                    tracing::info!("remove cache data: api:{}", api_key);
                }
            }
            drop(res_data);
        }
        Some(d_key) => {
            match res_data.get_mut(api_key) {
                Some(v) => {
                    v.remove(d_key);
                    tracing::info!("remove cache data,api_key: {},api:{}", api_key, d_key);
                }
                None => {
                    res_data.remove(api_key);
                    tracing::info!("remove cache data: api_key:{}", api_key);
                }
            };
            drop(res_data);
        }
    }
}

pub struct Cache;

impl<E: Endpoint> Middleware<E> for Cache {
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
                        remove_cache_data(&ctx.path.clone(), related_api, None).await;
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
