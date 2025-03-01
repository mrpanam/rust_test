use std::sync::LazyLock;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

pub async fn connect() -> Result<(), Box<dyn std::error::Error>> {
    DB.connect::<Ws>("localhost:8000").await?;

    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    DB.use_ns("eric").use_db("Xone").await?;

    Ok(())
}
