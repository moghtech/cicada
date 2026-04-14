use axum::http::StatusCode;
use cicada_client::{
  api::{read::FindSecret, write::UpdateSecret},
  entities::{
    EncryptedData,
    secret::{SecretId, SecretListItem, SecretRecord},
  },
};
use mogh_error::AddStatusCode as _;
use mogh_error::anyhow::Context as _;
use surrealdb_types::SurrealValue;

use crate::db::DB;

pub async fn list_secrets() -> mogh_error::Result<Vec<SecretListItem>>
{
  DB.query(
    "
SELECT * OMIT data FROM Secret
ORDER BY name COLLATE ASC;",
  )
  .await
  .context("Failed to query database for secrets")?
  .take(0)
  .context("Failed to get secret query result")
  .map_err(Into::into)
}

pub async fn list_secrets_matching(
  names: Vec<String>,
) -> mogh_error::Result<Vec<SecretRecord>> {
  if names.is_empty() {
    return Ok(Vec::new());
  }
  DB.query(
    "
SELECT * FROM Secret
WHERE name IN $names
ORDER BY name COLLATE ASC;",
  )
  .bind(("names", names))
  .await
  .context("Failed to query database for secrets")?
  .take(0)
  .context("Failed to get secret query result")
  .map_err(Into::into)
}

pub async fn get_secret(
  secret_id: &str,
) -> mogh_error::Result<SecretRecord> {
  DB.select::<Option<SecretRecord>>(("Secret", secret_id))
    .await
    .context("Failed to query database for secret")?
    .context("No secret found with given ID")
    .status_code(StatusCode::NOT_FOUND)
}

pub async fn find_secret(
  body: FindSecret,
) -> mogh_error::Result<SecretRecord> {
  DB.query(
    "
SELECT * FROM Secret
WHERE name = $name",
  )
  .bind(("name", body.name))
  .await
  .context("Failed to query database")?
  .take::<Option<SecretRecord>>(0)
  .context("Failed to get query result")?
  .context("Failed to find Secret with given parameters.")
  .status_code(StatusCode::NOT_FOUND)
}

#[derive(SurrealValue)]
pub struct CreateSecretQuery {
  pub name: String,
  pub description: String,
}

pub async fn create_secret(
  body: CreateSecretQuery,
) -> mogh_error::Result<SecretRecord> {
  DB.create("Secret")
    .content(body)
    .await
    .context("Failed to create Secret on database")?
    .context(
      "Failed to create Secret on database: No creation result",
    )
    .map_err(Into::into)
}

pub async fn update_secret(
  body: UpdateSecret,
) -> mogh_error::Result<SecretRecord> {
  DB.query("UPDATE $id MERGE fn::object_strip_none($body);")
    .bind(("id", body.id.clone()))
    .bind(("body", body))
    .await
    .context("Failed to query database")?
    .take::<Option<SecretRecord>>(0)
    .context("Failed to get query result")?
    .context("Failed to find secret with given parameters.")
    .status_code(StatusCode::NOT_FOUND)
}

pub async fn update_secret_data(
  id: SecretId,
  data: Option<EncryptedData>,
) -> mogh_error::Result<SecretRecord> {
  #[derive(SurrealValue)]
  struct UpdateSecretDataQuery {
    data: Option<EncryptedData>,
  }
  DB.update(id.as_record_id())
    .merge(UpdateSecretDataQuery { data })
    .await
    .context("Failed to update Secret on database")?
    .context("Failed to update Secret on database: No update result")
    .status_code(StatusCode::NOT_FOUND)
}

pub async fn delete_secret(
  id: String,
) -> mogh_error::Result<SecretRecord> {
  DB.delete(("Secret", id))
    .await?
    .context("No Secret matching given ID")
    .status_code(StatusCode::NOT_FOUND)
}

pub async fn batch_delete_secrets(
  ids: Vec<SecretId>,
) -> mogh_error::Result<Vec<SecretRecord>> {
  DB.query("DELETE Secret WHERE id IN $ids RETURN BEFORE;")
    .bind(("ids", ids))
    .await
    .context("Failed to delete secrets")?
    .take(0)
    .context("Invalid delete secrets query response")
    .map_err(Into::into)
}
