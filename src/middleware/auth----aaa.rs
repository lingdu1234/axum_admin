use crate::utils::{
    jwt::{Claims, KEYS},
    CasbinService,
};
use headers::{authorization::Bearer, HeaderMapExt};
use jsonwebtoken::{decode, errors::ErrorKind, Validation};
use poem::{http::StatusCode, Endpoint, Error, Middleware, Request, Result};
use sea_orm_casbin_adapter::casbin::prelude::*;
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
    type Output = Result<E::Output>;
    async fn call(&self, req: Request) -> Self::Output {
        let req_path = req.uri().path().replacen("/", "", 1);
        if req_path == "system/login" {
            return Ok(self.ep.call(req).await);
        }
        if let Some(auth) = req.headers().typed_get::<headers::Authorization<Bearer>>() {
            //  验证token
            // let validation = Validation {validate_exp: true,..Validation::default()};
            let token_data =
                match decode::<Claims>(auth.0.token(), &KEYS.decoding, &Validation::default()) {
                    Ok(token) => token,
                    Err(err) => {
                        match *err.kind() {
                            ErrorKind::InvalidToken => {
                                return Err(Error::new(StatusCode::UNAUTHORIZED)
                                    .with_reason("Invalid token"));
                            }
                            ErrorKind::ExpiredSignature => {
                                return Err(Error::new(StatusCode::UNAUTHORIZED)
                                    .with_reason("Expired token"));
                            }
                            _ => {
                                return Err(Error::new(StatusCode::UNAUTHORIZED)
                                    .with_reason(err.to_string()));
                            }
                        }
                    }
                };
            //  验证token是否过期
            // 验证casbin权限
            let e = req.extensions().get::<CasbinService>().unwrap();
            e.enforcer.enforce((
                &token_data.claims.id.to_string(),
                &req.uri().path().to_string(),
                &req.method().as_str(),
            ));
            //  if !casbin::is_permitted(&token_data.claims.role, req.path(), req.method()) {}
            println!("{:?}------req_path-{}", token_data.claims, req_path);
            return Ok(self.ep.call(req).await);
        }
        Err(Error::new(StatusCode::UNAUTHORIZED))
    }
}
