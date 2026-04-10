use std::sync::LazyLock;

use mogh_error::anyhow::Context as _;
use surrealdb::{Surreal, engine::any::Any, opt::auth};

use crate::config::core_config;

pub mod query;

pub static DB: LazyLock<Surreal<Any>> = LazyLock::new(Surreal::init);

pub async fn init() -> mogh_error::Result<()> {
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

  // All tables depend on this one
  DB.query(include_str!("tables/functions/timestamps.surrealql"))
    .await
    .context("Failed to initialize define timestamps function.")?;

  // Tables utilizing encrypted fields depend on this one
  DB.query(include_str!("tables/EncryptionKey.surrealql"))
    .await
    .context("Failed to initialize EncryptionKey table")?;
  DB.query(include_str!("tables/functions/encrypted_data.surrealql"))
    .await
    .context(
      "Failed to initialize define encrypted_data_field function.",
    )?;

  DB.query(include_str!("tables/User.surrealql"))
    .await
    .context("Failed to initialize User table")?;
  DB.query(include_str!("tables/ExternalLogin.surrealql"))
    .await
    .context("Failed to initialize ExternalLogin table")?;
  DB.query(include_str!("tables/Device.surrealql"))
    .await
    .context("Failed to initialize Device table")?;
  DB.query(include_str!("tables/OnboardingKey.surrealql"))
    .await
    .context("Failed to initialize OnboardingKey table")?;
  DB.query(include_str!("tables/Filesystem.surrealql"))
    .await
    .context("Failed to initialize Filesystem table")?;
  DB.query(include_str!("tables/Node.surrealql"))
    .await
    .context("Failed to initialize Node table")?;
  DB.query(include_str!("tables/Secret.surrealql"))
    .await
    .context("Failed to initialize Secret table")?;
  DB.query(include_str!("tables/Policy.surrealql"))
    .await
    .context("Failed to initialize Policy table")?;

  // List groups depends on the tables
  DB.query(include_str!("tables/functions/list_groups.surrealql"))
    .await
    .context(
      "Failed to initialize define list_groups function.",
    )?;

  Ok(())
}
