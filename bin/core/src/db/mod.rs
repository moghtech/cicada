use std::sync::LazyLock;

use anyhow::Context as _;
use surrealdb::{Surreal, engine::any::Any, opt::auth};

mod tables;

pub static DB: LazyLock<Surreal<Any>> = LazyLock::new(Surreal::init);

pub async fn init() -> anyhow::Result<()> {
  DB.connect("wss://surreal.van")
    .await
    .context("Failed to connect to database")?;

  DB.signin(auth::Root {
    username: String::from("root"),
    password: String::from("root"),
  })
  .await?;

  DB.use_ns("cicada").use_db("cicada").await?;

  DB.query(tables::INIT_FILESYSTEM_TABLE)
    .await
    .context("Failed to initialize filesystem table")?;
  DB.query(tables::INIT_NODE_TABLE)
    .await
    .context("Failed to initialize node table")?;

  Ok(())
}
