use axum::{Router, Json, routing::post, extract::State, response::IntoResponse, http::StatusCode};
use serde::{Deserialize, Serialize};
use mongodb::{bson::doc, Collection};
use crate::models::user::User;
use crate::auth::{create_jwt};
use bcrypt::{hash, verify, DEFAULT_COST};

#[derive(Deserialize)]
pub struct RegisterPayload {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
}

#[derive(Serialize)]
pub struct ErrorMsg {
    pub message: String,
}

use std::sync::Arc;
use crate::db::MongoDB;

pub fn user_routes() -> Router<Arc<MongoDB>> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

async fn register(State(db): State<Arc<MongoDB>>, Json(payload): Json<RegisterPayload>) -> (StatusCode, axum::Json<TokenResponse>) {
    let users: Collection<User> = db.users_collection();
    if users.find_one(doc! {"username": &payload.username}).await.unwrap().is_some() {
        return (StatusCode::BAD_REQUEST, Json(TokenResponse { token: "用户名已存在".to_string() }));
    }
    let hashed = hash(&payload.password, DEFAULT_COST).unwrap();
    let user = User {
        id: mongodb::bson::oid::ObjectId::new(),
        username: payload.username,
        password: hashed,
        created_at: mongodb::bson::DateTime::now(),
    };
    let res = users.insert_one(&user).await;
    match res {
        Ok(_) => {
            let token = create_jwt(&user.id);
            (StatusCode::OK, Json(TokenResponse { token }))
        },
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(TokenResponse { token: "注册失败".to_string() })),
    }
}

async fn login(State(db): State<Arc<MongoDB>>, Json(payload): Json<LoginPayload>) -> (StatusCode, axum::Json<TokenResponse>) {
    let users: Collection<User> = db.users_collection();
    let user_opt = users.find_one(doc! {"username": &payload.username}).await.unwrap();
    if let Some(user) = user_opt {
        if verify(&payload.password, &user.password).unwrap() {
            let token = create_jwt(&user.id);
            return (StatusCode::OK, Json(TokenResponse { token }));
        }
    }
    (StatusCode::UNAUTHORIZED, Json(TokenResponse { token: "用户名或密码错误".to_string() }))
}
