use poem::{async_trait, Endpoint, IntoResponse, Middleware, Request, Response};
use tracing::trace;

//  函数式中间件
pub async fn log<E: Endpoint>(next: E, req: Request) -> Response {
    trace!("请求地址:request: {}", req.uri().path());
    let resp = next.call(req).await.into_response();
    if resp.status().is_success() {
        println!("response: {}", resp.status());
    } else {
        println!("error: {}", resp.status());
    }

    resp
}

//  第二种中间件

pub struct Logger;

impl<E: Endpoint> Middleware<E> for Logger {
    type Output = LogImp<E>;

    fn transform(&self, ep: E) -> Self::Output {
        LogImp(ep)
    }
}

pub struct LogImp<E: Endpoint>(E);

#[async_trait]
impl<E: Endpoint> Endpoint for LogImp<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> Self::Output {
        tracing::info!("请求地址2:request: {}", req.uri().path());
        let resp = self.0.call(req).await.into_response();
        if resp.status().is_success() {
            println!("response--------2: {}", resp.status());
        } else {
            println!("error----2: {}", resp.status());
        }
        resp
    }
}
