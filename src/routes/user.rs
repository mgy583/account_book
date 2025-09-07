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
    match users.find_one(doc! {"username": &payload.username}).await {
        Ok(user_opt) => {
            if let Some(user) = user_opt {
                match verify(&payload.password, &user.password) {
                    Ok(valid) => {
                        if valid {
                            let token = create_jwt(&user.id);
                            return (StatusCode::OK, Json(TokenResponse { token }));
                        } else {
                            println!("[登录] 密码错误: {}", &payload.username);
                        }
                    },
                    Err(e) => {
                        println!("[登录] 密码校验异常: {}", e);
                        return (StatusCode::INTERNAL_SERVER_ERROR, Json(TokenResponse { token: "密码校验异常".to_string() }));
                    }
                }
            } else {
                println!("[登录] 用户不存在: {}", &payload.username);
            }
            (StatusCode::UNAUTHORIZED, Json(TokenResponse { token: "用户名或密码错误".to_string() }))
        },
        Err(e) => {
            println!("[登录] 数据库查询异常: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(TokenResponse { token: "数据库查询异常".to_string() }))
        }
    }
}
