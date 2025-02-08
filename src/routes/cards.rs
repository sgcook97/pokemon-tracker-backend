use crate::{models::cards::Card, AppState};
use axum::{
    extract::State,
    http::StatusCode,
    response::{self, IntoResponse},
    routing::get,
    Router,
};

use diesel::prelude::*;
use serde_json::json;

pub fn card_routes() -> Router<AppState> {
    Router::new().route("/", get(get_random_cards))
}

async fn get_random_cards(State(state): State<AppState>) -> impl IntoResponse {
    use crate::schema::cards::dsl::*;

    let conn = &mut state.db_pool.get().expect("Failed to get DB connection");

    match cards
        .order_by(diesel::dsl::sql::<diesel::sql_types::Float>("RANDOM()"))
        .limit(20)
        .load::<Card>(conn)
    {
        Ok(cards_data) => (
            StatusCode::OK,
            response::Json(json!({ "cards": cards_data })),
        ),
        Err(err) => {
            eprintln!("Database query failed: {:?}", err); // Better error logging
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                response::Json(json!({
                    "error": "Error retrieving cards from database."
                })),
            )
        }
    }
}
