mod db;
mod handlerEntity;
mod model;
mod query_runner;

use axum::{
    routing::{delete, get, post, put},
    Router,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting server...");
    db::connect().await?;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Listener bound");

    let app = Router::new()
        .route("/", get(handlerEntity::hello_world))
        .route("/books", get(handlerEntity::list_book))
        .route("/entities", get(handlerEntity::list_entities))
        .route("/entity/{id}", get(handlerEntity::get_entity))
        .route("/entity", post(handlerEntity::create_entity))
        .route("/entity/{id}", delete(handlerEntity::delete_entity))
        .route("/entity/{id}", put(handlerEntity::update_entity));

    axum::serve(listener, app).await?;
    println!("Server running");

    Ok(())
}
