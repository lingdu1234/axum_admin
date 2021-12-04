use chrono::{Duration, Utc};
use headers::{authorization::Bearer, Authorization};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use poem::{
    http::StatusCode,
    web::{Json, TypedHeader},
    FromRequest, IntoResponse, Request, RequestBody, Response,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::CFG;

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = &CFG.jwt.jwt_secret;
    Keys::new(secret.as_bytes())
});

#[derive(Debug)]
pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey<'static>,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret).into_static(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    pub id: String,
    pub name: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: String,
    pub name: String,
    pub exp: i64,
}

#[poem::async_trait]
impl<'a> FromRequest<'a> for Claims {
    type Error = AuthError;
    /// 将用户信息注意request
    async fn from_request(req: &'a Request, body: &mut RequestBody) -> Result<Self, Self::Error> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req, body)
                .await
                .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

pub async fn authorize(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, AuthError> {
    if payload.id.is_empty() || payload.name.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    println!("CFG.jwt.jwt_exp  == {:?}", CFG.jwt.jwt_exp);
    let iat = Utc::now();
    let exp = iat + Duration::minutes(CFG.jwt.jwt_exp);
    let claims = Claims {
        id: payload.id.to_owned(),
        name: payload.name,
        exp: exp.timestamp(),
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    // Send the authorized token
    Ok(Json(AuthBody::new(token, claims.exp)))
}

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}
impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
    exp: i64,
}
impl AuthBody {
    fn new(access_token: String, exp: i64) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
            exp,
        }
    }
}
