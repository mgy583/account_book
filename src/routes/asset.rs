use axum::{extract::State, Json, Router, routing::{get, post}};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::MongoDB;
use crate::auth::AuthUser;
use crate::models::asset::Asset;
use crate::routes::account::ApiError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAsset {
    pub name: String,
    pub asset_type: String,
    pub value: f64,
    pub currency: String,
    pub account_id: String,
    pub remark: Option<String>,
}

pub async fn create_asset_handler(
    State(db): State<Arc<MongoDB>>,
    AuthUser(user_id): AuthUser,
    Json(payload): Json<CreateAsset>,
) -> Result<Json<Asset>, ApiError> {
    let account_id = mongodb::bson::oid::ObjectId::parse_str(&payload.account_id).map_err(|e| ApiError { message: e.to_string() })?;
    let asset = db.create_asset(
        user_id,
        payload.name,
        payload.asset_type,
        payload.value,
        payload.currency,
        account_id,
        payload.remark,
    ).await?;
    Ok(Json(asset))
}

pub async fn get_assets_handler(
    State(db): State<Arc<MongoDB>>,
    AuthUser(user_id): AuthUser,
) -> Result<Json<Vec<Asset>>, ApiError> {
    let assets = db.get_assets_by_user(user_id).await?;
    Ok(Json(assets))
}

pub fn asset_routes() -> Router<Arc<MongoDB>> {
    Router::new()
        .route("/assets", post(create_asset_handler).get(get_assets_handler))
}
