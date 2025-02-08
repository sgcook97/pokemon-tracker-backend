use std::env;

use axum::http::{
    header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Method,
};
use backend::db::connection::establish_connection;
use backend::routes;
use backend::AppState;
use dotenvy::dotenv;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let frontend_url = env::var("FRONTEND_URL").expect("Unable to load FRONTEND_URL");

    let pool = establish_connection();

    let cors_layer = CorsLayer::new()
        .allow_origin(frontend_url.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .allow_credentials(true);

    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let state: AppState = AppState { db_pool: pool };
    let app = routes::create_router()
        .with_state(state)
        .layer(cors_layer)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
