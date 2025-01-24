pub mod auth;
pub mod cards;
pub mod sets;
pub mod users;

use axum::{response::Json, routing::get, Router};
use serde_json::{json, Value};

use crate::AppState;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::auth_routes())
        .nest("/sets", sets::set_routes())
        .route("/", get(index_route))
        .route("/health", get(health_check))
}

async fn index_route() -> Json<Value> {
    Json(json!({ "message": "Welcome to the API" }))
}

async fn health_check() -> Json<Value> {
    Json(json!({ "message": "Health check successful" }))
}
