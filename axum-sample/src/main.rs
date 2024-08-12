
mod handlers;
mod models;
mod db;


use axum::{
    routing::{delete, get, post, put},
    Router,
};
use mongodb::Database;
use std::sync::Arc;


#[tokio::main]
async fn main() {

    // Initialize the MongoDB client
    let db_client = db::init_db().await.expect("Fialed to initialize db.");

    let state = Arc::new(AppState { db_client });

    let app = Router::new()
        .route("/users", post(handlers::create_user))
        .route("/users", get(handlers::get_all_users))
        .route("/users/:id", get(handlers::get_user))
        .route("/users/:id", put(handlers::update_user))
        .route("/users/:id", delete(handlers::delete_user))
        .with_state(state);

    println!("Server running on http://localhost:3000");
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}



struct AppState {
    db_client: Database,
}

