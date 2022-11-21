use axum::{
    extract::{FromRequest, RequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{Duration, Local};
use db::common::ctx::UserInfoCtx;
use jsonwebtoken::{decode, encode, errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::apps::system::check_user_online;

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = &CFG.jwt.jwt_secret;
    Keys::new(secret.as_bytes())
});
use configs::CFG;

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthPayload {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub id: String,
    pub token_id: String,
    pub name: String,
    pub exp: i64,
}

#[axum::async_trait]
impl<B> FromRequest<B> for Claims
where
    B: Send,
{
    type Rejection = AuthError;
    /// 将用户信息注入request
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let (_, token_v) = get_bear_token(req).await?;
        // Decode the user data

        let token_data = match decode::<Claims>(&token_v, &KEYS.decoding, &Validation::default()) {
            Ok(token) => {
                let token_id = token.claims.token_id.clone();
                let (x, _) = check_user_online(None, token_id.clone()).await;
                if x {
                    token
                } else {
                    return Err(AuthError::CheckOutToken);
                }
            }
            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => {
                    return Err(AuthError::InvalidToken);
                }
                ErrorKind::ExpiredSignature => {
                    return Err(AuthError::MissingCredentials);
                }
                _ => {
                    return Err(AuthError::WrongCredentials);
                }
            },
        };
        let user = token_data.claims;
        req.extensions_mut().insert(UserInfoCtx {
            id: user.id.clone(),
            token_id: user.token_id.clone(),
            name: user.name.clone(),
        });
        Ok(user)
    }
}

pub async fn get_bear_token<B>(req: &mut RequestParts<B>) -> Result<(String, String), AuthError>
where
    B: Send,
{
    let TypedHeader(Authorization(bearer)) = TypedHeader::<Authorization<Bearer>>::from_request(req).await.map_err(|_| AuthError::InvalidToken)?;
    // Decode the user data
    let bearer_data = bearer.token();
    let cut = bearer_data.len() - scru128::new_string().len();
    Ok((bearer_data[cut..].to_string(), bearer_data[0..cut].to_string()))
}

pub async fn authorize(payload: AuthPayload, token_id: String) -> Result<AuthBody, AuthError> {
    if payload.id.is_empty() || payload.name.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    let iat = Local::now();
    let exp = iat + Duration::minutes(CFG.jwt.jwt_exp);
    let claims = Claims {
        id: payload.id.to_owned(),
        token_id: token_id.clone(),
        name: payload.name,
        exp: exp.timestamp(),
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding).map_err(|_| AuthError::WrongCredentials)?;
    // Send the authorized token
    Ok(AuthBody::new(token, claims.exp, CFG.jwt.jwt_exp, token_id))
}

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
    CheckOutToken,
}
impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
            AuthError::CheckOutToken => (StatusCode::UNAUTHORIZED, "该账户已经退出"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthBody {
    token: String,
    token_type: String,
    pub exp: i64,
    exp_in: i64,
}
impl AuthBody {
    fn new(access_token: String, exp: i64, exp_in: i64, token_id: String) -> Self {
        Self {
            token: access_token + &token_id,
            token_type: "Bearer".to_string(),
            exp,
            exp_in,
        }
    }
}
