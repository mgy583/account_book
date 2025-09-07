use axum::extract::FromRequestParts;
use axum::http::{request::Parts, StatusCode};
// use headers::{Authorization, authorization::Bearer};
// use async_trait::async_trait;
use jsonwebtoken::{decode, DecodingKey, Validation, encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
// use std::env;
use mongodb::bson::oid::ObjectId;

const SECRET: &[u8] = b"finance_secret_key";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub exp: usize,
}

pub fn create_jwt(user_id: &ObjectId) -> String {
    let claims = Claims {
        sub: user_id.to_hex(),
        exp: chrono::Utc::now().timestamp() as usize + 60 * 60 * 24 * 7, // 7天
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET)).unwrap()
}

pub fn decode_jwt(token: &str) -> Option<ObjectId> {
    let data = decode::<Claims>(token, &DecodingKey::from_secret(SECRET), &Validation::default()).ok()?;
    ObjectId::parse_str(&data.claims.sub).ok()
}

pub struct AuthUser(pub ObjectId);

use axum::http::header::AUTHORIZATION;

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);
    fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> impl futures::Future<Output = Result<Self, <Self as FromRequestParts<S>>::Rejection>> + Send {
        let headers = parts.headers.clone();
        async move {
            let auth_header = headers.get(AUTHORIZATION)
                .and_then(|v| v.to_str().ok())
                .ok_or((StatusCode::UNAUTHORIZED, "缺少或无效的Token".to_string()))?;
            let token = auth_header.strip_prefix("Bearer ")
                .ok_or((StatusCode::UNAUTHORIZED, "Token无效".to_string()))?;
            let user_id = decode_jwt(token).ok_or((StatusCode::UNAUTHORIZED, "Token无效".to_string()))?;
            Ok(AuthUser(user_id))
        }
    }
}
