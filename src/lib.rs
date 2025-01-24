pub mod db;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod schema;
pub mod utils;

use crate::db::connection::DbPool;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: DbPool,
}
