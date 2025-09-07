use axum::{Router};
mod auth;
use std::sync::Arc;
use tower_http::cors::{CorsLayer, Any};
mod db;
mod models;
mod routes;
use db::MongoDB;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("[启动] 理财系统服务启动中...");
    let db = MongoDB::new("mongodb://localhost:27017", "finance").await?;
    println!("[启动] MongoDB 连接成功，数据库: finance");
    let db = Arc::new(db);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
        .allow_headers([axum::http::header::CONTENT_TYPE]);

    let app = Router::new()
        .nest("/api", routes::api::api_routes())
        .with_state(db)
        .layer(cors);

    let addr = "0.0.0.0:3000";
    println!("[启动] 服务监听地址: http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("[启动] 服务已启动，等待请求...");
    axum::serve(listener, app).await?;
    Ok(())
}