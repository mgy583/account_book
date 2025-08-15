use tower_http::cors::{CorsLayer, Any};
use tracing::info;
use axum::{
    Json, Router, extract::{State, Path}, http::{StatusCode, header::{HeaderValue, CONTENT_TYPE}}, response::IntoResponse,
    response::Response, routing::get,
    routing::delete,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
mod db;
use db::{MongoDB};

#[derive(Deserialize, Serialize, Clone, Debug)]
struct CreateTodo {
    text: String,
}

// error
#[derive(Serialize, Debug)]
struct AppError {
    message: String,
}

impl<E: Into<anyhow::Error>> From<E> for AppError {
    fn from(error: E) -> Self {
        AppError {
            message: error.into().to_string(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = axum::Json(self);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            [(CONTENT_TYPE, HeaderValue::from_static("application/json; charset=utf-8"))],
            body
        ).into_response()
    }
}

async fn list_todos(State(db): State<Arc<MongoDB>>) -> Result<impl IntoResponse, AppError> {
    info!("GET /todos");
    let todos = db.get_todos().await?;
    Ok((
        StatusCode::OK,
        [(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8"),
        )],
        Json(todos),
    ))
}

async fn create_todo(
    State(db): State<Arc<MongoDB>>,
    Json(payload): Json<CreateTodo>,
) -> Result<impl IntoResponse, AppError> {
    info!("POST /todos");
    let todo = db.create_todo(payload.text).await?;
    Ok((
        StatusCode::CREATED,
        [(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8"),
        )],
        Json(todo),
    ))
}

async fn delete_todo(
    State(db): State<Arc<MongoDB>>,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    info!("DELETE /todos/{}", id);
    let oid = mongodb::bson::oid::ObjectId::parse_str(&id)
        .map_err(|_| AppError {
            message: "Invalid ID format".to_string(),
        })?;
    
    db.delete_todo(&oid).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // 初始化 MongoDB
    let db = MongoDB::new("mongodb://localhost:27017", "todo_app")
        .await
        .map_err(|e| anyhow::anyhow!(e))?;
    let db = Arc::new(db);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::DELETE,
        ])
        .allow_headers([axum::http::header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/todos", get(list_todos).post(create_todo))
        .route("/todos/{id}", delete(delete_todo))
        .with_state(db)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}