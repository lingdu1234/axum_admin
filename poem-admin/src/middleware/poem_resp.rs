use poem::{
    async_trait, web::Json, Endpoint, EndpointExt, IntoResponse, Middleware, Request, Response,
};
use serde::{Deserialize, Serialize};

pub struct PoemResp;

#[derive(Debug, Serialize, Deserialize)]
struct PoemRespData<T> {
    code: i32,
    msg: String,
    data: Json<T>,
}

impl<E: Endpoint> Middleware<E> for PoemResp {
    type Output = PoemRespImp<E>;

    fn transform(&self, ep: E) -> Self::Output {
        PoemRespImp { ep }
    }
}

pub struct PoemRespImp<E: Endpoint> {
    ep: E,
}

#[async_trait]
impl<E: EndpointExt> EndpointExt for PoemRespImp<E> {
    type Output = E::Output;

    async fn call(&self, res: Response) -> Self::Output {
        tracing::info!("请求地址2:request: {}", res.uri().path());
        let mut resp = self.0.call(res).await.into_response();
        if resp.status().is_success() {
            resp = PoemRespData {
                code: 0,
                msg: "success".to_string(),
                data: Json(resp),
            };
        } else {
            println!("error----2: {}", resp.status());
        }
        resp
    }
}
