use cicada_client::{
  api::write::{CreateUser, UpdateUser},
  entities::{
    external_login::{ExternalLoginKind, ExternalLoginRecord},
    user::{UserEntity, UserId, UserListItem, UserRecord},
  },
};
use mogh_auth_client::passkey::Passkey;
use mogh_error::{AddStatusCode, StatusCode, anyhow::Context as _};
use surrealdb_types::{SurrealValue, object};

use crate::db::DB;

pub async fn list_all_users() -> mogh_error::Result<Vec<UserListItem>>
{
  DB.select("User")
    .await
    .context("Failed to query for Users")
    .map_err(Into::into)
}

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
    groups: user.groups,
    external_skip_2fa: user.external_skip_2fa,
    admin: user.admin,
    super_admin: user.super_admin,
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

pub async fn sign_up_external_user(
  body: CreateUser,
  kind: ExternalLoginKind,
  external_id: String,
) -> mogh_error::Result<String> {
  let user = DB
    .query(
      "
      BEGIN TRANSACTION;
      let $user = CREATE ONLY User CONTENT $body; $user;
      CREATE ExternalLogin SET user = $user.id, kind = $kind, external_id = $external_id RETURN NONE;
      COMMIT TRANSACTION;",
    )
    .bind(("body", body))
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

#[derive(Default, SurrealValue)]
pub struct UpdateUserFields {
  pub name: Option<String>,
  pub enabled: Option<bool>,
  pub password: Option<String>,
  pub totp_secret: Option<String>,
  pub external_skip_2fa: Option<bool>,
}

pub async fn update_user_fields(
  id: String,
  body: UpdateUserFields,
) -> mogh_error::Result<UserRecord> {
  DB.query("UPDATE $id MERGE fn::object_strip_none($body);")
    .bind(("id", UserId(id)))
    .bind(("body", body))
    .await
    .context("Failed to query database")?
    .take::<Option<UserRecord>>(0)
    .context("Failed to get query result")?
    .context("Failed to find user with given parameters.")
    .status_code(StatusCode::NOT_FOUND)
}

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

pub async fn create_user(
  body: CreateUser,
) -> mogh_error::Result<UserRecord> {
  DB.create("User")
    .content(body)
    .await
    .context("Failed to create User on database")?
    .context("Failed to create User on database: No creation result")
    .map_err(Into::into)
}

pub async fn update_user(
  body: UpdateUser,
) -> mogh_error::Result<UserRecord> {
  DB.query("UPDATE $id MERGE fn::object_strip_none($body);")
    .bind(("id", body.id.clone()))
    .bind(("body", body))
    .await
    .context("Failed to query database")?
    .take::<Option<UserRecord>>(0)
    .context("Failed to get query result")?
    .context("Failed to find user with given parameters.")
    .status_code(StatusCode::NOT_FOUND)
}

pub async fn delete_user(
  id: UserId,
) -> mogh_error::Result<UserRecord> {
  DB.delete(id.as_record_id())
    .await?
    .context("No User matching given ID")
    .status_code(StatusCode::NOT_FOUND)
}

pub async fn batch_delete_users(
  ids: Vec<UserId>,
) -> mogh_error::Result<Vec<UserRecord>> {
  DB.query("DELETE User WHERE id IN $ids RETURN BEFORE;")
    .bind(("ids", ids))
    .await
    .context("Failed to delete users")?
    .take(0)
    .context("Invalid delete users query response")
    .map_err(Into::into)
}
