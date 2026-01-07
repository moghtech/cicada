use anyhow::Context as _;
use cicada_client::entities::user::UserRecord;

use crate::db::DB;

pub async fn get_user(user_id: &str) -> anyhow::Result<UserRecord> {
  DB.select::<Option<UserRecord>>(("User", user_id))
    .await
    .context("Failed to query database for user")?
    .context("No user found with given ID")
}

pub async fn find_user_with_username(
  username: String,
) -> anyhow::Result<UserRecord> {
  DB.query("SELECT * FROM User WHERE name = $name")
    .bind(("name", username))
    .await
    .context("Failed to query database for user")?
    .take::<Vec<UserRecord>>(0)
    .context("Failed to deserialize UserRecord")?
    .pop()
    .context("Did not find user with given username")
}

pub async fn sign_up_local_user(
  username: String,
  hashed_password: String,
  enabled: bool,
) -> anyhow::Result<String> {
  let user = DB
    .query("CREATE User SET name = $name, password = $password, enabled = $enabled;")
    .bind(("name", username))
    .bind(("password", hashed_password))
    .bind(("enabled", enabled))
    .await
    .context("Failed to create user on database")?
    .take::<Option<UserRecord>>(0)
    .context("Failed to deserialize UserRecord")?
    .context("Query response missing created UserRecord")?;
  Ok(user.id.0)
}
