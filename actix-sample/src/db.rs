use mongodb::{Client, options::ClientOptions, Database};
use std::error::Error;

pub async fn init_db() -> Result<Database, Box<dyn Error>> {

    // Parse the MongoDB connection string
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

    // Create the MongoDB client
    let client = Client::with_options(client_options)?;

    // Get the database
    Ok(client.database("rust_actix_users_db"))

}