use anyhow::Context as _;
use cicada_client::entities::user::UserRecord;
use mogh_auth_client::passkey::Passkey;
use serde::Serialize;
use surrealdb_types::object;

use crate::db::DB;

pub async fn get_user(user_id: &str) -> anyhow::Result<UserRecord> {
  DB.select::<Option<UserRecord>>(("User", user_id))
    .await
    .context("Failed to query database for user")?
    .context("No user found with given ID")
}

pub async fn no_users_exist() -> anyhow::Result<bool> {
  let no_users = DB
    .query("SELECT * FROM User LIMIT 1;")
    .await
    .context("Failed to query database for user")?
    .take::<Vec<UserRecord>>(0)
    .context("Failed to deserialize UserRecord")?
    .pop()
    .is_none();
  Ok(no_users)
}

pub async fn find_user_with_username(
  username: String,
) -> anyhow::Result<Option<UserRecord>> {
  let user = DB
    .query("SELECT * FROM User WHERE name = $name")
    .bind(("name", username))
    .await
    .context("Failed to query database for user")?
    .take::<Vec<UserRecord>>(0)
    .context("Failed to deserialize UserRecord")?
    .pop();
  Ok(user)
}

pub async fn find_user_with_oidc_subject(
  oidc_subject: String,
) -> anyhow::Result<Option<UserRecord>> {
  let user = DB
    .query("SELECT * FROM User WHERE oidc_subject = $oidc_subject")
    .bind(("oidc_subject", oidc_subject))
    .await
    .context("Failed to query database for user")?
    .take::<Vec<UserRecord>>(0)
    .context("Failed to deserialize UserRecord")?
    .pop();
  Ok(user)
}

pub async fn find_user_with_github_id(
  github_id: String,
) -> anyhow::Result<Option<UserRecord>> {
  let user = DB
    .query("SELECT * FROM User WHERE github_id = $github_id")
    .bind(("github_id", github_id))
    .await
    .context("Failed to query database for user")?
    .take::<Vec<UserRecord>>(0)
    .context("Failed to deserialize UserRecord")?
    .pop();
  Ok(user)
}

pub async fn find_user_with_google_id(
  google_id: String,
) -> anyhow::Result<Option<UserRecord>> {
  let user = DB
    .query("SELECT * FROM User WHERE google_id = $google_id")
    .bind(("google_id", google_id))
    .await
    .context("Failed to query database for user")?
    .take::<Vec<UserRecord>>(0)
    .context("Failed to deserialize UserRecord")?
    .pop();
  Ok(user)
}

pub async fn sign_up_local_user(
  username: String,
  hashed_password: String,
  enabled: bool,
) -> anyhow::Result<String> {
  let user = DB
    .query("CREATE User SET name = $name, enabled = $enabled, password = $password;")
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

pub async fn sign_up_oidc_user(
  username: String,
  oidc_subject: String,
  enabled: bool,
) -> anyhow::Result<String> {
  let user = DB
    .query("CREATE User SET name = $name, enabled = $enabled, oidc_subject = $oidc_subject;")
    .bind(("name", username))
    .bind(("oidc_subject", oidc_subject))
    .bind(("enabled", enabled))
    .await
    .context("Failed to create user on database")?
    .take::<Option<UserRecord>>(0)
    .context("Failed to deserialize UserRecord")?
    .context("Query response missing created UserRecord")?;
  Ok(user.id.0)
}

pub async fn sign_up_github_user(
  username: String,
  github_id: String,
  enabled: bool,
) -> anyhow::Result<String> {
  let user = DB
    .query("CREATE User SET name = $name, enabled = $enabled, github_id = $github_id;")
    .bind(("name", username))
    .bind(("github_id", github_id))
    .bind(("enabled", enabled))
    .await
    .context("Failed to create user on database")?
    .take::<Option<UserRecord>>(0)
    .context("Failed to deserialize UserRecord")?
    .context("Query response missing created UserRecord")?;
  Ok(user.id.0)
}

pub async fn sign_up_google_user(
  username: String,
  google_id: String,
  enabled: bool,
) -> anyhow::Result<String> {
  let user = DB
    .query("CREATE User SET name = $name, enabled = $enabled, google_id = $google_id;")
    .bind(("name", username))
    .bind(("google_id", google_id))
    .bind(("enabled", enabled))
    .await
    .context("Failed to create user on database")?
    .take::<Option<UserRecord>>(0)
    .context("Failed to deserialize UserRecord")?
    .context("Query response missing created UserRecord")?;
  Ok(user.id.0)
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateUser {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub enabled: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub password: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub oidc_subject: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub github_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub google_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub totp_secret: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub external_skip_2fa: Option<bool>,
}

/// Uses merge strategy for non optional field types.
/// If the field is None, it will not be updated.
pub async fn update_user_fields(
  id: String,
  update: UpdateUser,
) -> anyhow::Result<UserRecord> {
  DB.update(("User", id))
    .merge(serde_json::to_value(update)?)
    .await
    .context("Failed to query database")?
    .context("No user update result")
}

/// Because passkey is Option type,
/// need it's own 'content' type update
/// which can set it to NONE.
pub async fn update_user_passkey(
  id: String,
  passkey: Option<Passkey>,
) -> anyhow::Result<UserRecord> {
  let passkey: Option<serde_json::Value> =
    if let Some(passkey) = passkey {
      serde_json::from_str(&serde_json::to_string(&passkey)?)?
    } else {
      None
    };
  DB.update(("User", id))
    .merge(object! { "passkey": passkey })
    .await
    .context("Failed to query database")?
    .context("No user update result")
}
