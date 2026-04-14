use axum::http::StatusCode;
use cicada_client::{
  api::write::UpdateEncryptionKey,
  entities::encryption_key::{
    EncryptionKeyId, EncryptionKeyKind, EncryptionKeyRecord,
  },
};
use mogh_error::AddStatusCode as _;
use mogh_error::anyhow::Context as _;
use surrealdb_types::SurrealValue;

use crate::db::DB;

pub async fn list_all_encryption_keys()
-> mogh_error::Result<Vec<EncryptionKeyRecord>> {
  DB.query("SELECT * FROM EncryptionKey ORDER BY created_at ASC;")
    .await
    .context("Failed to query for EncryptionKeys")?
    .take(0)
    .context("Failed to get EncryptionKey query result")
    .map_err(Into::into)
}

pub async fn get_encryption_key(
  encryption_key_id: &str,
) -> mogh_error::Result<EncryptionKeyRecord> {
  DB.select::<Option<EncryptionKeyRecord>>((
    "EncryptionKey",
    encryption_key_id,
  ))
  .await
  .context("Failed to query database for encryption key")?
  .context("No encryption key found with given ID")
  .status_code(StatusCode::NOT_FOUND)
}

#[derive(SurrealValue)]
pub struct CreateEncryptionKeyQuery {
  pub name: String,
  pub kind: EncryptionKeyKind,
  pub key: Option<String>,
  pub verification: String,
  pub verification_encrypted: String,
  pub verification_nonce: String,
}

pub async fn create_encryption_key(
  body: CreateEncryptionKeyQuery,
) -> mogh_error::Result<EncryptionKeyRecord> {
  DB.create("EncryptionKey")
    .content(body)
    .await
    .context("Failed to create EncryptionKey on database")?
    .context(
      "Failed to create EncryptionKey on database: No creation result",
    )
    .map_err(Into::into)
}

pub async fn update_encryption_key(
  body: UpdateEncryptionKey,
) -> mogh_error::Result<EncryptionKeyRecord> {
  DB.query("UPDATE $id MERGE fn::object_strip_none($body);")
    .bind(("id", body.id.clone()))
    .bind(("body", body))
    .await
    .context("Failed to query database")?
    .take::<Option<EncryptionKeyRecord>>(0)
    .context("Failed to get query result")?
    .context("Failed to find encryption key with given parameters.")
    .status_code(StatusCode::NOT_FOUND)
}

pub async fn delete_encryption_key(
  id: String,
) -> mogh_error::Result<EncryptionKeyRecord> {
  DB.delete(("EncryptionKey", id))
    .await?
    .context("No EncryptionKey matching given ID")
    .status_code(StatusCode::NOT_FOUND)
}

pub async fn batch_delete_encryption_keys(
  ids: Vec<EncryptionKeyId>,
) -> mogh_error::Result<Vec<EncryptionKeyRecord>> {
  DB.query("DELETE EncryptionKey WHERE id IN $ids RETURN BEFORE;")
    .bind(("ids", ids))
    .await
    .context("Failed to delete encryption keys")?
    .take(0)
    .context("Invalid delete encryption keys query response")
    .map_err(Into::into)
}
