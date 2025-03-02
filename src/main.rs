mod db;
mod handlers;
mod model;
mod query_runner;

use axum::{
    routing::{delete, get, post, put},
    Router,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let number: f64 = 1123456789.89;
    println!("Starting server...");
    println!("{}", number.to_string());

    db::connect().await?;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Listener bound");

    let app = Router::new()
        .route("/", get(handlers::hello_world))
        .route("/books", get(handlers::list_book))
        .route("/entities", get(handlers::list_entities))
        .route("/entity/{id}", get(handlers::get_entity))
        .route("/entity", post(handlers::create_entity))
        .route("/entity/{id}", delete(handlers::delete_entity))
        .route("/entity/{id}", put(handlers::update_entity));

    axum::serve(listener, app).await?;
    println!("Server running");

    Ok(())
}
