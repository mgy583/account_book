use axum::{extract::State, Json, Router, routing::{get, post}, http::StatusCode, response::IntoResponse};
use crate::auth::AuthUser;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::MongoDB;
use crate::models::account::Account;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAccount {
    pub name: String,
    pub account_type: String,
    pub balance: f64,
    pub currency: String,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub message: String,
}

impl<E: std::fmt::Display> From<E> for ApiError {
    fn from(e: E) -> Self {
        ApiError { message: e.to_string() }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let body = Json(self);
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

pub async fn create_account_handler(
    State(db): State<Arc<MongoDB>>,
    AuthUser(user_id): AuthUser,
    Json(payload): Json<CreateAccount>,
) -> Result<Json<Account>, ApiError> {
    let account = db.create_account(
        user_id,
        payload.name,
        payload.account_type,
        payload.balance,
        payload.currency,
        payload.remark,
    ).await?;
    Ok(Json(account))
}

pub async fn get_accounts_handler(
    State(db): State<Arc<MongoDB>>,
    AuthUser(user_id): AuthUser,
) -> Result<Json<Vec<Account>>, ApiError> {
    let accounts = db.get_accounts_by_user(user_id).await?;
    Ok(Json(accounts))
}

pub fn account_routes() -> Router<Arc<MongoDB>> {
    Router::new()
        .route("/accounts", post(create_account_handler).get(get_accounts_handler))
}
