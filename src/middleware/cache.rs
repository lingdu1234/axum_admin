use core::time::Duration;
use std::{collections::HashMap, sync::Arc, time::Instant};

use configs::CFG;
use db::common::res::ResJsonString;
use once_cell::sync::Lazy;
use poem::{Endpoint, IntoResponse, Middleware, Request, Response, Result};
use tokio::sync::Mutex;

use crate::utils::jwt;

pub static RES_DATA: Lazy<Arc<Mutex<HashMap<String, ResData>>>> = Lazy::new(|| {
    let data: HashMap<String, ResData> = HashMap::new();
    tokio::spawn(async { self::init().await });
    Arc::new(Mutex::new(data))
});

#[derive(Clone, Debug)]
pub struct ResData {
    pub time: Instant,
    pub data: String,
}

pub async fn init() {
    tracing::info!("cache data init");
    let d = CFG.server.cache_time * 1000;
    loop {
        tokio::time::sleep(Duration::from_millis(30 * 1000)).await;

        let mut res_data = RES_DATA.lock().await;

        for (k, v) in res_data.clone().iter() {
            if Instant::now().duration_since(v.time).as_millis() as u64 > d {
                res_data.remove(k);
                tracing::info!("remove cache data: {}", k);
            }
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
                Ok(v) => Ok(v.into_response()),
                Err(e) => Err(e),
            };
        }
        let ori_uri = req.original_uri().to_string();

        let key = ori_uri.clone() + &method + &token_id;
        // 开始请求数据
        let mut res_data = RES_DATA.lock().await;
        match res_data.get(&key) {
            Some(v) => {
                let data = v.data.clone();
                Ok(data.into_response())
            }
            None => {
                let res_end = self.inner.call(req).await;
                match res_end {
                    Ok(v) => {
                        let res = v.into_response();
                        let res_ctx = match res.extensions().get::<ResJsonString>() {
                            Some(x) => x.0.clone(),
                            None => "".to_string(),
                        };
                        res_data.insert(
                            key.clone(),
                            ResData {
                                time: Instant::now(),
                                data: res_ctx,
                            },
                        );
                        tracing::info!("add cache data: {}", key);
                        Ok(res)
                    }
                    Err(e) => Err(e),
                }
            }
        }
    }
}
