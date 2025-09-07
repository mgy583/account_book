use axum::{extract::State, Json, Router, routing::post};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::MongoDB;
use crate::auth::AuthUser;
use crate::models::transaction::Order;
use crate::routes::account::ApiError;


#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrder {
    pub order_type: String, // 消费/收入/转账
    pub amount: f64,
    pub currency: String,   // 币种
    pub date: String,       // 日期字符串
    pub remark: Option<String>,
}

pub async fn create_order_handler(
    State(db): State<Arc<MongoDB>>,
    AuthUser(user_id): AuthUser,
    Json(payload): Json<CreateOrder>,
) -> Result<Json<Order>, ApiError> {
    // 校验类型和币种
    let allowed_types = ["消费", "收入", "转账"];
    let allowed_currencies = ["人民币", "美元", "欧元"];
    if !allowed_types.contains(&payload.order_type.as_str()) {
        return Err(ApiError { message: "类型不合法".to_string() });
    }
    if !allowed_currencies.contains(&payload.currency.as_str()) {
        return Err(ApiError { message: "币种不合法".to_string() });
    }
    let date = mongodb::bson::DateTime::parse_rfc3339_str(&payload.date).map_err(|e| ApiError { message: e.to_string() })?;
    let order = db.create_order(
        user_id,
        payload.order_type,
        payload.amount,
        payload.currency,
        date,
        payload.remark,
    ).await?;
    Ok(Json(order))
}

pub async fn get_orders_handler(
    State(db): State<Arc<MongoDB>>,
    AuthUser(user_id): AuthUser,
) -> Result<Json<Vec<Order>>, ApiError> {
    let orders = db.get_orders_by_user(user_id).await?;
    Ok(Json(orders))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteOrderPayload {
    pub id: String,
}

pub async fn delete_order_handler(
    State(db): State<Arc<MongoDB>>,
    AuthUser(user_id): AuthUser,
    Json(payload): Json<DeleteOrderPayload>,
) -> Result<Json<bool>, ApiError> {
    let order_id = mongodb::bson::oid::ObjectId::parse_str(&payload.id).map_err(|e| ApiError { message: e.to_string() })?;
    let deleted = db.delete_order(user_id, order_id).await?;
    Ok(Json(deleted))
}

pub fn order_routes() -> Router<Arc<MongoDB>> {
    Router::new()
        .route("/orders", post(create_order_handler).get(get_orders_handler))
        .route("/orders/delete", post(delete_order_handler))
}
