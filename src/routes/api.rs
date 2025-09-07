use axum::{Router, routing::get};
use std::sync::Arc;
use crate::db::MongoDB;

pub fn api_routes() -> Router<Arc<MongoDB>> {
    Router::new()
        // 统一聚合各业务路由
    .nest("/account", crate::routes::account::account_routes())
    .nest("/category", crate::routes::category::category_routes())
    .nest("/asset", crate::routes::asset::asset_routes())
    .nest("/transaction", crate::routes::transaction::order_routes())
    .nest("/budget", crate::routes::budget::budget_routes())
    .nest("/user", crate::routes::user::user_routes())
    .nest("/order", crate::routes::order::order_routes())
    .nest("/order_query", crate::routes::order_query::order_query_routes())
}
