use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::entities::{
  EncryptedData, Iso8601Timestamp, encryption_key::EncryptionKeyId,
};

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SecretListItem {
  /// The unique secret id
  pub id: SecretId,
  /// The name of the secret
  pub name: String,
  /// Created at as ISO8601 timestamp.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub created_at: Iso8601Timestamp,
  /// Updated at as ISO8601 timestamp.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub updated_at: Iso8601Timestamp,
}

/// Secrets over the API, with unencrypted data
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SecretEntity {
  /// The unique secret id
  pub id: SecretId,
  /// The name of the secret
  pub name: String,
  /// The master encryption key for this secret, if set.
  /// If this is not null while data is, it means
  /// the encryption key is not initialized.
  pub encryption_key: Option<EncryptionKeyId>,
  /// Data associated with the secret.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data: Option<String>,
  /// Created at as ISO8601 timestamp.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub created_at: Iso8601Timestamp,
  /// Updated at as ISO8601 timestamp.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub updated_at: Iso8601Timestamp,
}

/// Secrets stored on the database, with encrypted data
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SecretRecord {
  /// The unique secret id
  pub id: SecretId,
  /// The name of the secret
  pub name: String,
  /// Data associated with the secret.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data: Option<EncryptedData>,
  /// Created at as ISO8601 timestamp.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub created_at: Iso8601Timestamp,
  /// Updated at as ISO8601 timestamp.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub updated_at: Iso8601Timestamp,
}

#[typeshare(serialized_as = "string")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SecretId(pub String);

crate::surreal_id!(SecretId, "Secret");
