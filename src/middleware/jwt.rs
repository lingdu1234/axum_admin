use crate::CFG;

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use poem::http::StatusCode;
use poem::web::{Json, TypedHeader};
use poem::{Endpoint, FromRequest, IntoResponse, Request, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;

// 从配置文件获取KEYS
static KEYS: Lazy<Keys> = Lazy::new(|| Keys::new((&CFG.jwt.jwt_secret).as_bytes()));

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

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user_name: String,
    user_id: String,
    exp: usize,
}

#[derive(Debug, Serialize)]
struct AuthBody {
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

#[derive(Debug, Deserialize)]
struct AuthPayload {
    client_id: String,
    client_secret: String,
}

#[derive(Debug)]
enum AuthError {
    WrongCredentials,   // 客户端ID或密钥错误
    MissingCredentials, // 缺少客户端ID或密钥
    TokenCreation,      //创建token
    InvalidToken,       //无效token
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
        (status, body).into()
    }
}

async fn authorize(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, AuthError> {
    // Check if the user sent the credentials
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    // Here you can check the user credentials from a database
    if payload.client_id != "foo" || payload.client_secret != "bar" {
        return Err(AuthError::WrongCredentials);
    }
    let claims = Claims {
        user_name: "b@b.com".to_owned(),
        user_id: "ACME".to_owned(),
        exp: 10000000000,
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    // Send the authorized token
    Ok(Json(AuthBody::new(token)))
}

// #[poem::async_trait]
// impl<E: Endpoint> Endpoint for Claims {
//     type Output = Result<E::Output, E>;
//
//     async fn call(&self, req: Request) -> Self::Output {
//         todo!()
//     }
//
//     // async fn from_request(req: &mut Request) -> Result<Self, Self::Error> {
//     //     // Extract the token from the authorization header
//     //     let TypedHeader(Authorization(bearer)) =
//     //         TypedHeader::<Authorization<Bearer>>::from_request(req)
//     //             .await
//     //             .map_err(|_| AuthError::InvalidToken)?;
//     //     // Decode the user data
//     //     let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
//     //         .map_err(|_| AuthError::InvalidToken)?;
//     //
//     //     Ok(token_data.claims)
//     // }
// }
