mod db;
mod models;
mod routes;

use mongodb::Database;
use rocket::{launch, routes};

struct AppState {
    db_client: Database,
}

#[launch]
async fn rocket() -> _ {

    // Initialize the MongoDB client
    let db_client = db::init_db().await.expect("Fialed to initialize db.");

    rocket::build()
        .manage(AppState { db_client })
        .mount("/", routes![
            routes::create_user,
            routes::get_user,
            routes::get_all_users,
            routes::update_user,
            routes::delete_user
        ])

}