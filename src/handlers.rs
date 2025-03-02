use crate::db::DB; // Import the DB variable
use crate::model::{Book, CreateEntityRequest, Entity}; // Import the Entity model and Id enum
use axum::extract::{Json, Path};
use axum::http::StatusCode;
use axum::response::Html;

pub async fn list_book() -> Result<Json<Vec<Book>>, StatusCode> {
    println!("before select");

    let book = DB
        .select("book")
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    println!("book: {:?}", book);
    Ok(Json(book))
}

pub async fn list_entities() -> Result<Json<Vec<Entity>>, axum::http::StatusCode> {
    println!("before select");

    let entities = DB
        .select("entity")
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    println!("entities: {:?}", entities);
    Ok(Json(entities))
}

// Add this new function for the root route
pub async fn hello_world() -> Html<&'static str> {
    Html("Hello, World!")
}

pub async fn get_entity(id: Path<String>) -> Result<Json<Option<Entity>>, axum::http::StatusCode> {
    let entity: Option<Entity> = DB
        .select(("entity", id.0.to_string()))
        .await
        .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;

    Ok(Json(entity))
}

pub async fn create_entity(
    Json(req): Json<CreateEntityRequest>,
) -> Result<Json<Entity>, StatusCode> {
    let record: Option<Entity> = DB
        .create(("entity", req.id))
        .content(Entity {
            id: None,
            name: req.name,
        })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(record.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?))
}

pub async fn delete_entity(id: Path<String>) -> Result<StatusCode, StatusCode> {
    let _deleted: Option<Entity> = DB
        .delete(("entity", id.0))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_entity(
    id: Path<String>,
    Json(req): Json<Entity>,
) -> Result<Json<Entity>, StatusCode> {
    let record: Option<Entity> = DB
        .update(("entity", id.0))
        .content(Entity {
            id: None,
            name: req.name,
        })
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(record.ok_or(StatusCode::NOT_FOUND)?))
}
