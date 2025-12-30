use std::sync::LazyLock;

use anyhow::Context as _;
use surrealdb::{Surreal, engine::any::Any, opt::auth};

use crate::config::core_config;

mod tables;

pub static DB: LazyLock<Surreal<Any>> = LazyLock::new(Surreal::init);

pub async fn init() -> anyhow::Result<()> {
  let config = core_config();

  DB.connect(&config.database.uri)
    .await
    .context("Failed to connect to database")?;

  if !config.database.password.is_empty() {
    DB.signin(auth::Root {
      username: config.database.username.clone(),
      password: config.database.password.clone(),
    })
    .await?;
  }

  DB.use_ns(&config.database.namespace)
    .use_db(&config.database.db_name)
    .await?;

  DB.query(tables::INIT_TABLES)
    .await
    .context("Failed to initialize tables")?;

  Ok(())
}
