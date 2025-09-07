use axum::{extract::{State, Query}, Json, Router, routing::get};
use std::sync::Arc;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::db::MongoDB;
use crate::auth::AuthUser;
use mongodb::bson::DateTime;
use serde_json;

#[derive(Debug, Deserialize)]
pub struct OrderQuery {
    pub page: Option<usize>,
    pub page_size: Option<usize>,
    pub name: Option<String>,
    pub order_type: Option<String>,
    pub date_start: Option<String>,
    pub date_end: Option<String>,
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

pub async fn query_orders_handler(
    State(db): State<Arc<MongoDB>>,
    AuthUser(user_id): AuthUser,
    Query(query): Query<OrderQuery>,
) -> Json<HashMap<&'static str, serde_json::Value>> {
    let mut db_orders = db.get_orders_by_user(user_id).await.unwrap_or_default();
    // 筛选
    if let Some(ref name) = query.name {
        db_orders = db_orders.into_iter().filter(|o| o.name.contains(name)).collect();
    }
    if let Some(ref t) = query.order_type {
        db_orders = db_orders.into_iter().filter(|o| o.order_type == *t).collect();
    }
    if let (Some(start), Some(end)) = (&query.date_start, &query.date_end) {
        if let (Ok(start), Ok(end)) = (DateTime::parse_rfc3339_str(start), DateTime::parse_rfc3339_str(end)) {
            db_orders = db_orders.into_iter().filter(|o| o.date >= start && o.date <= end).collect();
        }
    }
    let total = db_orders.len();
    // 分页
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(8);
    let start = (page-1)*page_size;
    let page_orders = db_orders.iter().skip(start).take(page_size).map(|o| Order {
        id: o.id.to_hex(),
        name: o.name.clone(),
        order_type: o.order_type.clone(),
        amount: o.amount,
        currency: o.currency.clone(),
        date: o.date.try_to_rfc3339_string().unwrap_or_default(),
        remark: o.remark.clone(),
    }).collect::<Vec<_>>();
    // 分类统计
    let mut stat: HashMap<String, f64> = HashMap::new();
    for o in &db_orders {
        *stat.entry(o.order_type.clone()).or_insert(0.0) += o.amount;
    }
    let mut result = HashMap::new();
    result.insert("total", serde_json::json!(total));
    result.insert("orders", serde_json::to_value(page_orders).unwrap());
    result.insert("stat", serde_json::to_value(stat).unwrap());
    Json(result)
}

pub fn order_query_routes() -> Router<Arc<MongoDB>> {
    Router::new()
        .route("/orders/query", get(query_orders_handler))
}
