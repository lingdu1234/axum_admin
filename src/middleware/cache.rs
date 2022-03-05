use core::time::Duration;
use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
    time::Instant,
};

use configs::CFG;
use db::common::res::ResJsonString;
use once_cell::sync::Lazy;
use poem::{Endpoint, IntoResponse, Middleware, Request, Response, Result};
use tokio::sync::Mutex;

use crate::utils::jwt;

pub static RES_DATA: Lazy<Arc<Mutex<HashMap<String, HashMap<String, String>>>>> = Lazy::new(|| {
    let data: HashMap<String, HashMap<String, String>> = HashMap::new();
    Arc::new(Mutex::new(data))
});

// 格式 token★apipath
pub static RES_BMAP: Lazy<Arc<Mutex<BTreeMap<String, Instant>>>> = Lazy::new(|| {
    let bmap: BTreeMap<String, Instant> = BTreeMap::new();
    tokio::spawn(async { self::init().await });
    Arc::new(Mutex::new(bmap))
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
    let mut res_bmap = RES_BMAP.lock().await;
    for (k, v) in res_bmap.clone().iter() {
        if Instant::now().duration_since(*v).as_millis() as u64 > d {
            let key = k.split('★').collect::<Vec<&str>>();
            remove_cache_data(key[0], Some(key[1])).await;
            res_bmap.remove(k);
        } else {
            break;
        }
    }
}

pub async fn add_cache_data(token_id: &str, api: &str, data: String) {
    let mut res_data = RES_DATA.lock().await;
    let mut res_bmap = RES_BMAP.lock().await;
    let key = format!("{}★{}", token_id, api);

    res_bmap.insert(key, Instant::now());
    let hmap: HashMap<String, String> = HashMap::new();
    let v = res_data.entry(token_id.to_string()).or_insert(hmap);
    v.insert(api.to_string(), data);
    tracing::info!("add cache data,token_id: {},api:{}", token_id, api);
}

pub async fn get_cache_data(token_id: &str, api: &str) -> Option<String> {
    let res_data = RES_DATA.lock().await;

    let h = match res_data.get(token_id) {
        Some(v) => v,
        None => {
            return None;
        }
    };
    h.get(api).map(|v| v.to_string())
}

pub async fn remove_cache_data(token_id: &str, api: Option<&str>) {
    let mut res_data = RES_DATA.lock().await;

    match api {
        None => {
            res_data.remove(token_id);
            tracing::info!("remove cache data: token_id:{}", token_id);
        }
        Some(api_v) => {
            match res_data.get_mut(token_id) {
                Some(v) => {
                    v.remove(api_v);
                    tracing::info!("remove cache data,token_id: {},api:{}", token_id, api_v);
                }
                None => {
                    res_data.remove(token_id);
                    tracing::info!("remove cache data: token_id:{}", token_id);
                }
            };
        }
    }
}

pub struct Cache;

impl<E: Endpoint> Middleware<E> for Cache {
    type Output = CacheEndpoint<E>;

    fn transform(&self, ep: E) -> Self::Output {
        CacheEndpoint { inner: ep }
    }
}

/// Endpoint for `Tracing` middleware.
pub struct CacheEndpoint<E> {
    inner: E,
}

#[poem::async_trait]
impl<E: Endpoint> Endpoint for CacheEndpoint<E> {
    // type Output = E::Output;
    type Output = Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        let (token_id, _) = jwt::get_bear_token(&req).await?;

        let method = req.method().to_string();
        if method.clone().as_str() != "GET" {
            let res_end = self.inner.call(req).await;
            return match res_end {
                Ok(v) => {
                    remove_cache_data(&token_id, None).await;
                    Ok(v.into_response())
                }
                Err(e) => Err(e),
            };
        }
        let ori_uri = req.original_uri().to_string();

        let key = ori_uri.clone() + &method;
        // 开始请求数据
        match get_cache_data(&token_id, &key).await {
            Some(v) => Ok(v.into_response()),

            None => {
                let res_end = self.inner.call(req).await;
                match res_end {
                    Ok(v) => {
                        let res = v.into_response();
                        let res_ctx = match res.extensions().get::<ResJsonString>() {
                            Some(x) => x.0.clone(),
                            None => "".to_string(),
                        };
                        // 缓存数据
                        add_cache_data(&token_id, &key, res_ctx).await;

                        Ok(res)
                    }
                    Err(e) => Err(e),
                }
            }
        }
    }
}

// 感觉没有什么鸟用
