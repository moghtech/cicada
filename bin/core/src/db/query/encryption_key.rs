use anyhow::Context as _;
use axum::http::StatusCode;
use cicada_client::{
  api::write::encryption_key::UpdateEncryptionKey,
  entities::encryption_key::{
    EncryptionKeyKind, EncryptionKeyRecord,
  },
};
use mogh_error::AddStatusCode as _;
use surrealdb_types::SurrealValue;

use crate::db::DB;

pub async fn list_all_encryption_keys()
-> anyhow::Result<Vec<EncryptionKeyRecord>> {
  DB.select("EncryptionKey")
    .await
    .context("Failed to query for EncryptionKeys")
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
) -> anyhow::Result<EncryptionKeyRecord> {
  DB.create("EncryptionKey")
    .content(body)
    .await
    .context("Failed to create EncryptionKey on database")?
    .context(
      "Failed to create EncryptionKey on database: No creation result",
    )
}

pub async fn update_encryption_key(
  body: UpdateEncryptionKey,
) -> anyhow::Result<EncryptionKeyRecord> {
  DB.update(body.id.as_record_id())
    .merge(serde_json::to_value(body)?)
    .await
    .context("Failed to update EncryptionKey on database")?
    .context(
      "Failed to update EncryptionKey on database: No update result",
    )
}
