mod handlers;
mod models;
mod db;
use actix_web::{web, App, HttpServer};


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Initialize the MongoDB client
    let db_client = db::init_db().await.expect("Fialed to initialize db.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_client.clone()))
            .service(handlers::create_user)
            .service(handlers::get_user)
            .service(handlers::get_all_users)
            .service(handlers::update_user)
            .service(handlers::delete_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
