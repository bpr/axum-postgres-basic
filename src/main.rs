mod handlers;
mod models;
mod routes;

use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    //expose environment variables from .env file
    dotenvy::dotenv().expect("Unable to access .env file");

    //set variables from enviroment variables
    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in env file");

    //create our database pool
    let db_pool = PgPoolOptions::new()
        .max_connections(64)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
        .expect("can't connect to database");

    //create our tcp listener
    let listener = TcpListener::bind(server_address)
        .await
        .expect("Could not create tcp listener");

    println!("listening on {}", listener.local_addr().unwrap());

    // compose the routes
    let app = routes::new().with_state(db_pool);

    //serve the application
    axum::serve(listener, app)
        .await
        .expect("Error serving application");
}
