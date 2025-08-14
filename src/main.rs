use tower_http::cors::{CorsLayer, Any};
use tracing::info;
use axum::{
    Json, Router, extract::{State}, http::{StatusCode, header::{HeaderValue, CONTENT_TYPE}}, response::IntoResponse,
    response::Response, routing::get,
    routing::delete,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

// model definitions
#[derive(Deserialize, Serialize, Clone, Debug)]
struct Todo {
    id: u32,
    text: String,
    done: bool,
}

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

//
async fn list_todos(State(db): State<Arc<Mutex<HashMap<u32, Todo>>>>) -> impl IntoResponse {
    info!("GET /todos");
    let db = db.lock().await;
    let todos = db.values().cloned().collect::<Vec<_>>();
    (
        StatusCode::OK,
        [(CONTENT_TYPE, HeaderValue::from_static("application/json; charset=utf-8"))],
        axum::Json(todos)
    )
        .into_response()
}

async fn create_todo(
    State(db): State<Arc<Mutex<HashMap<u32, Todo>>>>,
    Json(payload): Json<CreateTodo>,
    ) -> impl IntoResponse {
    info!("POST /todos");
    let mut db = db.lock().await;
    let id = db.len() as u32 + 1;
    let todo = Todo {
        id,
        text: payload.text,
        done: false,
    };
    db.insert(id, todo.clone());
    (
        StatusCode::CREATED,
        [(CONTENT_TYPE, HeaderValue::from_static("application/json; charset=utf-8"))],
        axum::Json(todo)
    )
        .into_response()
}

async fn delete_todo(
    State(db): State<Arc<Mutex<HashMap<u32, Todo>>>>,
    axum::extract::Path(id): axum::extract::Path<u32>,
) -> Result<StatusCode, AppError> {
    info!("DELETE /todos/{}", id);
    let mut db = db.lock().await;
    if db.remove(&id).is_some() {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError {
            message: "Todo not found".to_string(),
        })
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    // 内存数据库
    let db = Arc::new(Mutex::new(HashMap::<u32, _>::new()));


    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST, axum::http::Method::DELETE])
        .allow_headers([axum::http::header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/todos", get(list_todos).post(create_todo))
        .route("/todos/{id}", delete(delete_todo))
        .with_state(db)
        .layer(cors);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
