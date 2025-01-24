use crate::{
    middleware::auth_middleware,
    models::{cards::Card, sets::Set},
    AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::{self, IntoResponse},
    routing::get,
    Router,
};

use diesel::prelude::*;
use rand::Rng;
use serde_json::json;

pub fn set_routes() -> Router<AppState> {
    Router::new()
        .route("/{set_id}", get(set_by_id))
        .route("/{set_id}/cards", get(get_cards_by_set))
        .layer(middleware::from_fn(auth_middleware::verify_token))
        .route("/", get(get_random_set))
}

async fn set_by_id(
    State(state): State<AppState>,
    Path(other_set_id): Path<String>,
) -> impl IntoResponse {
    use crate::schema::sets::dsl::*;

    let conn = &mut state
        .db_pool
        .get()
        .expect("Failed to get a connection from the pool");

    match sets.filter(set_id.eq(other_set_id)).first::<Set>(conn) {
        Ok(set) => {
            return (
                StatusCode::OK,
                response::Json(json!({
                    "message": "Set successfully retrieved.",
                    "data": set
                })),
            );
        }
        Err(diesel::result::Error::NotFound) => {
            return (
                StatusCode::NOT_FOUND,
                response::Json(json!({
                    "error": "Set not found in database."
                })),
            );
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                response::Json(json!({
                    "error": "Error retrieving set information."
                })),
            );
        }
    }
}

async fn get_random_set(State(state): State<AppState>) -> impl IntoResponse {
    use crate::schema::sets::dsl::*;

    let conn = &mut state
        .db_pool
        .get()
        .expect("Failed to get a connection from the pool");

    let count: i64 = sets.count().get_result(conn).expect("Error counting sets");

    if count == 0 {
        return (
            StatusCode::NOT_FOUND,
            response::Json(json!({
                "error": "No sets found in database."
            })),
        );
    }

    let random_offset = rand::thread_rng().gen_range(0..count);

    match sets.offset(random_offset).first::<Set>(conn) {
        Ok(set) => {
            return (
                StatusCode::OK,
                response::Json(json!({
                    "message": "Set successfully retrieved.",
                    "data": set
                })),
            );
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                response::Json(json!({
                    "error": "Error retrieving set from database."
                })),
            );
        }
    }
}

async fn get_cards_by_set(
    State(state): State<AppState>,
    Path(other_set_id): Path<String>,
) -> impl IntoResponse {
    use crate::schema::cards::dsl::*;

    let conn = &mut state
        .db_pool
        .get()
        .expect("Failed to get a connection from the pool");

    match cards
        .filter(set_id.eq(other_set_id))
        .order(sort_key.asc())
        .load::<Card>(conn)
    {
        Ok(cards_data) => {
            return (
                StatusCode::OK,
                response::Json(json!({
                    "message": "Cards successfully retrieved.",
                    "data": cards_data
                })),
            );
        }
        Err(diesel::result::Error::NotFound) => {
            return (
                StatusCode::NOT_FOUND,
                response::Json(json!({
                    "error": "No cards found for the given set id."
                })),
            );
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                response::Json(json!({
                    "error": "Error retrieving cards from database."
                })),
            );
        }
    }
}
