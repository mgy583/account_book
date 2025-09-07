use serde::{Serialize, Deserialize};
use axum::{extract::State, Json, Router, routing::{post, delete}};

#[derive(Debug, Deserialize)]
pub struct OrderQuery {
    pub page: Option<usize>,
    pub page_size: Option<usize>,
    pub name: Option<String>,
    pub order_type: Option<String>,
    pub date_start: Option<String>,
    pub date_end: Option<String>,
}
use axum::extract::Json as AxumJson;
use std::sync::Arc;
use crate::db::MongoDB;
use crate::auth::AuthUser;
use mongodb::bson::DateTime;
use crate::models::transaction::Order as DbOrder;

#[derive(Debug, Deserialize)]
pub struct CreateOrder {
    pub name: String,
    #[serde(rename = "type")]
    pub order_type: String,
    pub amount: f64,
    pub currency: String,
    pub date: String,
    pub remark: Option<String>,
}

pub async fn create_order_handler(
    State(db): State<Arc<MongoDB>>,
    AuthUser(user_id): AuthUser,
    AxumJson(payload): AxumJson<CreateOrder>,
) -> Json<DbOrder> {
    println!("[INFO][create_order_handler] 收到前端payload: {:?}", payload);
    let date = DateTime::parse_rfc3339_str(&payload.date).unwrap_or_else(|_| DateTime::now());
    match db.create_order(
        user_id,
        payload.name.clone(),
        payload.order_type.clone(),
        payload.amount,
        payload.currency.clone(),
        date,
        payload.remark.clone(),
    ).await {
        Ok(db_order) => {
            println!("[INFO][create_order_handler] 数据库插入成功: {:?}", db_order);
            Json(db_order)
        },
        Err(e) => {
            println!("[ERROR][create_order_handler] 数据库插入失败: {:?}", e);
            panic!("db error: {:?}", e);
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub name: String,
    pub order_type: String,
    pub amount: f64,
    pub currency: String,
    pub date: String,
    pub remark: Option<String>,
}


pub fn order_routes() -> Router<Arc<MongoDB>> {
    println!("[INFO][order_routes] 订单路由已注册 /order");
    use crate::routes::order_delete::delete_order_handler;
    Router::new()
        .route("/", post(create_order_handler))
        .route("/{id}", delete(delete_order_handler))
}
