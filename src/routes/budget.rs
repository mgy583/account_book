use axum::{extract::State, Json, Router, routing::{get, post}};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::MongoDB;
use crate::auth::AuthUser;
use crate::models::budget::Budget;
use crate::routes::account::ApiError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBudget {
    pub category_id: String,
    pub amount: f64,
    pub period: String,
    pub start_date: String,
    pub end_date: String,
}

pub async fn create_budget_handler(
    State(db): State<Arc<MongoDB>>,
    AuthUser(user_id): AuthUser,
    Json(payload): Json<CreateBudget>,
) -> Result<Json<Budget>, ApiError> {
    let category_id = mongodb::bson::oid::ObjectId::parse_str(&payload.category_id).map_err(|e| ApiError { message: e.to_string() })?;
    let start_date = mongodb::bson::DateTime::parse_rfc3339_str(&payload.start_date).map_err(|e| ApiError { message: e.to_string() })?;
    let end_date = mongodb::bson::DateTime::parse_rfc3339_str(&payload.end_date).map_err(|e| ApiError { message: e.to_string() })?;
    let budget = db.create_budget(
        user_id,
        category_id,
        payload.amount,
        payload.period,
        start_date,
        end_date,
    ).await?;
    Ok(Json(budget))
}

pub async fn get_budgets_handler(
    State(db): State<Arc<MongoDB>>,
    AuthUser(user_id): AuthUser,
) -> Result<Json<Vec<Budget>>, ApiError> {
    let budgets = db.get_budgets_by_user(user_id).await?;
    Ok(Json(budgets))
}

pub fn budget_routes() -> Router<Arc<MongoDB>> {
    Router::new()
        .route("/budgets", post(create_budget_handler).get(get_budgets_handler))
}
