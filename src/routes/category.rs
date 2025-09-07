use axum::{extract::State, Json, Router, routing::{get, post}, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::MongoDB;
use crate::auth::AuthUser;
use crate::models::category::Category;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCategory {
    pub name: String,
    pub parent_id: Option<String>,
    pub category_type: String,
}

use crate::routes::account::ApiError;

pub async fn create_category_handler(
    State(db): State<Arc<MongoDB>>,
    AuthUser(user_id): AuthUser,
    Json(payload): Json<CreateCategory>,
) -> Result<Json<Category>, ApiError> {
    let parent_id = payload.parent_id.and_then(|id| mongodb::bson::oid::ObjectId::parse_str(&id).ok());
    let category = db.create_category(
        user_id,
        payload.name,
        parent_id,
        payload.category_type,
    ).await?;
    Ok(Json(category))
}

pub async fn get_categories_handler(
    State(db): State<Arc<MongoDB>>,
    AuthUser(user_id): AuthUser,
) -> Result<Json<Vec<Category>>, ApiError> {
    let categories = db.get_categories_by_user(user_id).await?;
    Ok(Json(categories))
}

pub fn category_routes() -> Router<Arc<MongoDB>> {
    Router::new()
        .route("/categories", post(create_category_handler).get(get_categories_handler))
}
