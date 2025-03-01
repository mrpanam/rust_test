use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    pub id: RecordId,
    pub entity: RecordId,
    pub active: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entity {
    pub id: Option<RecordId>,
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateEntityRequest {
    pub id: String,
    pub name: String,
}
