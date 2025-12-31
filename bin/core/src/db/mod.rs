use std::sync::LazyLock;

use anyhow::Context as _;
use surrealdb::{Surreal, engine::any::Any, opt::auth};

use crate::config::core_config;

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

  DB.query(include_str!("tables/Filesystem.surrealql"))
    .await
    .context("Failed to initialize Filesystem table")?;
  DB.query(include_str!("tables/Node.surrealql"))
    .await
    .context("Failed to initialize Node table")?;

  Ok(())
}
