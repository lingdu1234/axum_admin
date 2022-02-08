use headers::{authorization::Bearer, HeaderMapExt};
use jsonwebtoken::{decode, errors::ErrorKind, Validation};
use poem::{http::StatusCode, Endpoint, Error, Middleware, Request, Result};

use crate::{
    apps::system::check_user_online,
    utils::{
        jwt::{Claims, KEYS},
        ApiUtils,
    },
    CFG,
};

#[derive(Clone, Debug)]
pub struct Auth;

impl<E: Endpoint> Middleware<E> for Auth {
    type Output = AuthEndpoint<E>;

    fn transform(&self, ep: E) -> Self::Output {
        AuthEndpoint { ep }
    }
}

pub struct AuthEndpoint<E> {
    ep: E,
}

#[poem::async_trait]
impl<E: Endpoint> Endpoint for AuthEndpoint<E> {
    type Output = E::Output;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        let req_vec = req.original_uri().path().split('/').collect::<Vec<&str>>();
        let req_path = if req_vec.len() > 2 {
            req_vec[2..].join("/")
        } else {
            "".to_string()
        };
        let req_method = req.method().as_str();
        println!("{}   =======================   {}", req_path, req_method);
        if let Some(auth) = req.headers().typed_get::<headers::Authorization<Bearer>>() {
            //  验证token
            // let validation = Validation {validate_exp: true,..Validation::default()};
            let token_data =
                match decode::<Claims>(auth.0.token(), &KEYS.decoding, &Validation::default()) {
                    Ok(token) => {
                        let token_id = token.claims.token_id.clone();
                        let x = check_user_online(None, token_id).await;
                        if x {
                            token
                        } else {
                            return Err(Error::from_string(
                                "该账户已经退出",
                                StatusCode::UNAUTHORIZED,
                            ));
                        }
                    }
                    Err(err) => match *err.kind() {
                        ErrorKind::InvalidToken => {
                            return Err(Error::from_string(
                                "Invalid token",
                                StatusCode::UNAUTHORIZED,
                            ));
                        }
                        ErrorKind::ExpiredSignature => {
                            return Err(Error::from_string(
                                "Expired token",
                                StatusCode::UNAUTHORIZED,
                            ));
                        }
                        _ => {
                            return Err(Error::from_string(
                                err.to_string(),
                                StatusCode::UNAUTHORIZED,
                            ));
                        }
                    },
                };
            // 如果是超级用户，则不需要验证权限，直接放行
            if CFG.system.super_user.contains(&token_data.claims.id) {
                return self.ep.call(req).await;
            }

            // 验证api权限，如果不在路由表中，则放行，否则验证权限
            if !req_path.is_empty() {
                if ApiUtils::is_in(&req_path).await {
                    if ApiUtils::check_api_permission(&req_path, req_method).await {
                        return self.ep.call(req).await;
                    } else {
                        return Err(Error::from_string(
                            "你没有权限访问该页面",
                            StatusCode::UNAUTHORIZED,
                        ));
                    }
                } else {
                    return self.ep.call(req).await;
                }
                return self.ep.call(req).await;
            }
        }
        Err(Error::from_status(StatusCode::UNAUTHORIZED))
    }
}
