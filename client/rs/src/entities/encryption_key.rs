use serde::{Deserialize, Serialize};
use surrealdb_types::{RecordId, RecordIdKey, SurrealValue};
use typeshare::typeshare;

use crate::entities::{
  Iso8601Timestamp, master_key::MasterKeyId,
  record_id::CicadaRecordId,
};

/// Record fields are encrypted using encryption keys stored
/// in [EncryptionKeyRecord]. These keys are themselves encrypted using
/// a master key.
///
/// This pattern allows both record keys and master keys to be rotated.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct EncryptionKeyRecord {
  /// The unique encryption key id
  pub id: EncryptionKeyId,
  /// The record which this key encrypts.
  pub record: CicadaRecordId,
  /// Master key used to encrypt this key.
  pub master: MasterKeyId,
  /// Encrypted with master key, base64 encoded.
  pub key: String,
  /// Key encryption nonce, base64 encoded, or empty string.
  ///
  /// If non-empty, is the nonce used to encrypt this key using
  /// the master key.
  pub nonce: String,

  // ==
  // TODO: Format and version fields
  // ==
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
pub struct EncryptionKeyId(pub String);

crate::surreal_id!(EncryptionKeyId, "EncryptionKey");
