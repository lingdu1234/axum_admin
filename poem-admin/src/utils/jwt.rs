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

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = &CFG.jwt.jwt_secret;
    Keys::new(secret.as_bytes())
});

#[derive(Debug)]
struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey<'static>,
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
    pub client_id: String,
    pub client_secret: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

#[poem::async_trait]
impl<'a> FromRequest<'a> for Claims {
    type Error = AuthError;

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
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    let claims = Claims {
        sub: payload.client_id.to_owned(),
        company: payload.client_secret.to_owned(),
        exp: 10000000000,
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    // Send the authorized token
    Ok(Json(AuthBody::new(token)))
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
}
impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}
