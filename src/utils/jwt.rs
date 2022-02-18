use chrono::{Duration, Local};
use headers::{authorization::Bearer, Authorization};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use poem::{http::StatusCode, web::TypedHeader, Error, FromRequest, Request, RequestBody, Result};
use serde::{Deserialize, Serialize};

use crate::CFG;

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = &CFG.jwt.jwt_secret;
    Keys::new(secret.as_bytes())
});

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

#[poem::async_trait]
impl<'a> FromRequest<'a> for Claims {
    // type Error = AuthError;
    /// 将用户信息注入request
    async fn from_request(req: &'a Request, body: &mut RequestBody) -> Result<Self> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req, body)
                .await
                .map_err(|_| Error::from_string("InvalidToken", StatusCode::BAD_REQUEST))?;
        // Decode the user data
        let token_data =
            decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
                .map_err(|_| Error::from_string("InvalidToken", StatusCode::BAD_REQUEST))?;

        Ok(token_data.claims)
    }
    async fn from_request_without_body(req: &'a Request) -> Result<Self> {
        Self::from_request(req, &mut Default::default()).await
    }
}

pub async fn authorize(payload: AuthPayload, token_id: String) -> Result<AuthBody> {
    if payload.id.is_empty() || payload.name.is_empty() {
        return Err(Error::from_string(
            "Missing credentials",
            StatusCode::BAD_REQUEST,
        ));
    }
    let iat = Local::now();
    let exp = iat + Duration::minutes(CFG.jwt.jwt_exp);
    let claims = Claims {
        id: payload.id.to_owned(),
        token_id,
        name: payload.name,
        exp: exp.timestamp(),
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding).map_err(|_| {
        Error::from_string("Token creation error", StatusCode::INTERNAL_SERVER_ERROR)
    })?;

    // Send the authorized token
    Ok(AuthBody::new(token, claims.exp, CFG.jwt.jwt_exp))
}

// #[derive(Debug)]
// pub enum AuthError {
//     WrongCredentials,
//     MissingCredentials,
//     TokenCreation,
//     InvalidToken,
// }
// impl IntoResponse for AuthError {
//     fn into_response(self) -> Response {
//         let (status, error_message) = match self {
//             AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong
// credentials"),             AuthError::MissingCredentials =>
// (StatusCode::BAD_REQUEST, "Missing credentials"),
// AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token
// creation error"),             AuthError::InvalidToken =>
// (StatusCode::BAD_REQUEST, "Invalid token"),         };
//         let body = Json(json!({
//             "error": error_message,
//         }));
//         (status, body).into_response()
//     }
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthBody {
    token: String,
    token_type: String,
    pub exp: i64,
    exp_in: i64,
}
impl AuthBody {
    fn new(access_token: String, exp: i64, exp_in: i64) -> Self {
        Self {
            token: access_token,
            token_type: "Bearer".to_string(),
            exp,
            exp_in,
        }
    }
}
