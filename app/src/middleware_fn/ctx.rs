use axum::{
    body::Body,
    extract::OriginalUri,
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use bytes::Bytes;
use configs::CFG;
use db::common::ctx::ReqCtx;

/// req上下文注入中间件 同时进行jwt授权验证
pub async fn ctx_fn_mid(req: Request<Body>, next: Next<Body>) -> Result<impl IntoResponse, (StatusCode, String)> {
    // 请求信息ctx注入

    let ori_uri_path = if let Some(path) = req.extensions().get::<OriginalUri>() {
        path.0.path().to_owned()
    } else {
        req.uri().path().to_owned()
    };
    let path = ori_uri_path.replacen(&(CFG.server.api_prefix.clone() + "/"), "", 1);
    let method = req.method().to_string();
    let path_params = req.uri().query().unwrap_or("").to_string();

    let (parts, req_body) = req.into_parts();

    let (bytes, body_data) = match get_body_data(req_body).await {
        Err(e) => return Err(e),
        Ok((x, y)) => (x, y),
    };

    let req_ctx = ReqCtx {
        ori_uri: if path_params.is_empty() { ori_uri_path } else { ori_uri_path + "?" + &path_params },
        path,
        path_params,
        method: method.to_string(),
        // user: UserInfo {
        //     id: user.id,
        //     token_id: user.token_id,
        //     name: user.name,
        // },
        data: body_data.clone(),
    };

    let mut req = Request::from_parts(parts, Body::from(bytes));

    req.extensions_mut().insert(req_ctx);

    let res = next.run(req).await;
    Ok(res)
}

//  获取body数据
async fn get_body_data<B>(body: B) -> Result<(Bytes, String), (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match hyper::body::to_bytes(body).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err((StatusCode::BAD_REQUEST, format!("failed to read body: {}", err)));
        }
    };

    match std::str::from_utf8(&bytes) {
        Ok(x) => {
            let res_data = x.to_string();
            Ok((bytes, res_data))
        }
        Err(_) => Ok((bytes, "该数据无法转输出，可能为blob，binary".to_string())),
    }
}
