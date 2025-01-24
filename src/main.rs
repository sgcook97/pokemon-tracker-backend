use backend::db::connection::establish_connection;
use backend::routes;
use backend::AppState;

#[tokio::main]
async fn main() {
    let pool = establish_connection();

    let state: AppState = AppState { db_pool: pool };
    let app = routes::create_router().with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
