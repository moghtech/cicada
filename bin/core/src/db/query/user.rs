use cicada_client::entities::{
  external_login::{ExternalLoginKind, ExternalLoginRecord},
  user::{UserEntity, UserId, UserRecord},
};
use mogh_auth_client::passkey::Passkey;
use mogh_error::{AddStatusCode, StatusCode, anyhow::Context as _};
use serde::Serialize;
use surrealdb_types::object;

use crate::db::DB;

pub async fn get_user_entity(
  user_id: String,
) -> mogh_error::Result<UserEntity> {
  let mut res = DB
    .query(
      "
  SELECT * FROM ONLY $user;
  SELECT * FROM ExternalLogin WHERE user = $user;",
    )
    .bind(("user", UserId(user_id)))
    .await
    .context("Failed to query database for user entity")?;
  let user = res
    .take::<Option<UserRecord>>(0)
    .context("Invalid user query response")?
    .context("No user found at given ID")
    .status_code(StatusCode::NOT_FOUND)?;
  let external_logins = res
    .take::<Vec<ExternalLoginRecord>>(1)
    .context("Invalid external login query response")?;
  Ok(UserEntity {
    id: user.id,
    username: user.username,
    avatar: user.avatar,
    enabled: user.enabled,
    password: !user.password.is_empty(),
    external_logins,
    passkey: user.passkey.is_some(),
    totp: !user.totp_secret.is_empty(),
    external_skip_2fa: user.external_skip_2fa,
    created_at: user.created_at,
    updated_at: user.updated_at,
  })
}

pub async fn get_user(
  user_id: &str,
) -> mogh_error::Result<UserRecord> {
  DB.select::<Option<UserRecord>>(("User", user_id))
    .await
    .context("Failed to query database for user")?
    .context("No user found with given ID")
    .status_code(StatusCode::NOT_FOUND)
}

pub async fn no_users_exist() -> mogh_error::Result<bool> {
  let no_users = DB
    .query("SELECT * FROM ONLY User LIMIT 1;")
    .await
    .context("Failed to query database for user")?
    .take::<Option<UserRecord>>(0)
    .context("Failed to deserialize UserRecord")?
    .is_none();
  Ok(no_users)
}

pub async fn find_user_with_username(
  username: String,
) -> mogh_error::Result<Option<UserRecord>> {
  DB.query("SELECT * FROM ONLY User WHERE username = $username;")
    .bind(("username", username))
    .await
    .context("Failed to query database for user")?
    .take(0)
    .context("Failed to deserialize UserRecord")
    .map_err(Into::into)
}

pub async fn find_user_with_external_login(
  kind: ExternalLoginKind,
  external_id: String,
) -> mogh_error::Result<Option<UserRecord>> {
  DB.query(
    "
    SELECT VALUE user.* FROM ONLY ExternalLogin
    WHERE kind = $kind AND external_id = $external_id;",
  )
  .bind(("kind", kind))
  .bind(("external_id", external_id))
  .await
  .context("Failed to query database for user")?
  .take(0)
  .context("Failed to deserialize UserRecord")
  .map_err(Into::into)
}

pub async fn sign_up_local_user(
  username: String,
  hashed_password: String,
  enabled: bool,
) -> mogh_error::Result<String> {
  let user = DB
    .query("CREATE ONLY User SET username = $username, password = $password, enabled = $enabled;")
    .bind(("username", username))
    .bind(("password", hashed_password))
    .bind(("enabled", enabled))
    .await
    .context("Failed to query database to sign up user")?
    .take::<Option<UserRecord>>(0)
    .context("Failed to deserialize UserRecord")?
    .context("Query response missing created UserRecord")?;
  Ok(user.id.0)
}

pub async fn sign_up_external_user(
  username: String,
  avatar: String,
  kind: ExternalLoginKind,
  external_id: String,
  enabled: bool,
) -> mogh_error::Result<String> {
  let user = DB
    .query(
      "
    BEGIN TRANSACTION;
    let $user = CREATE ONLY User SET username = $username, avatar = $avatar, enabled = $enabled; $user;
    CREATE ExternalLogin SET user = $user.id, kind = $kind, external_id = $external_id RETURN NONE;
    COMMIT TRANSACTION;",
    )
    .bind(("username", username))
    .bind(("avatar", avatar))
    .bind(("enabled", enabled))
    .bind(("kind", kind))
    .bind(("external_id", external_id))
    .await
    .context("Failed to query database to sign up external user")?
    .take::<Option<UserRecord>>(2)
    .context("Failed to deserialize UserRecord")?
    .context("Query response missing created UserRecord")?;
  Ok(user.id.0)
}

pub async fn link_external_login(
  user_id: String,
  kind: ExternalLoginKind,
  external_id: String,
) -> mogh_error::Result<ExternalLoginRecord> {
  DB.query("CREATE ONLY ExternalLogin SET user = $user, kind = $kind, external_id = $external_id;")
    .bind(("user", UserId(user_id)))
    .bind(("kind", kind))
    .bind(("external_id", external_id))
    .await
    .context("Failed to query database for external login")?
    .take::<Option<ExternalLoginRecord>>(0)
    .context("Failed to deserialize ExternalLoginRecord")?
    .context("Missing external login creation response.")
    .status_code(StatusCode::NOT_FOUND)
}

pub async fn unlink_external_login(
  user_id: String,
  kind: ExternalLoginKind,
) -> mogh_error::Result<ExternalLoginRecord> {
  DB.query("DELETE ExternalLogin WHERE user = $user AND kind = $kind RETURN BEFORE;")
    .bind(("user", UserId(user_id)))
    .bind(("kind", kind))
    .await
    .context("Failed to query database for external login")?
    .take::<Vec<ExternalLoginRecord>>(0)
    .context("Failed to deserialize ExternalLoginRecord")?
    .pop()
    .context("Missing external login deletion response.")
    .status_code(StatusCode::NOT_FOUND)
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
  pub totp_secret: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub external_skip_2fa: Option<bool>,
}

/// Uses merge strategy for non optional field types.
/// If the field is None, it will not be updated.
pub async fn update_user_fields(
  id: String,
  update: UpdateUser,
) -> mogh_error::Result<UserRecord> {
  DB.update(("User", id))
    .merge(serde_json::to_value(update)?)
    .await
    .context("Failed to query database")?
    .context("No user update result")
    .status_code(StatusCode::NOT_FOUND)
}

/// Because passkey is Option type,
/// need it's own 'content' type update
/// which can set it to NONE.
pub async fn update_user_passkey(
  id: String,
  passkey: Option<Passkey>,
) -> mogh_error::Result<UserRecord> {
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
    .status_code(StatusCode::NOT_FOUND)
}
