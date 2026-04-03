use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};
use surrealdb_types::{SerializationError, SurrealValue};
use typeshare::typeshare;

use crate::entities::Iso8601Timestamp;

/// Record fields are encrypted by storing them as [EncryptedData] type.
/// These keys are themselves encrypted using an [EncryptionKeyRecord],
/// which can point to an in-memory key or a remote KMS.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct EncryptionKeyEntity {
  /// The unique encryption key id
  pub id: EncryptionKeyId,
  /// The name of the encryption key. Must be unique.
  pub name: String,
  /// The kind of encryption key.
  /// - Memory
  /// - Disk
  pub kind: EncryptionKeyKind,
  /// For Memory keys:
  /// Whether the key has been initialized.
  pub initialized: bool,
  /// Created at as ISO8601 timestamp.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub created_at: Iso8601Timestamp,
  /// Updated at as ISO8601 timestamp.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub updated_at: Iso8601Timestamp,
}

/// Record fields are encrypted by storing them as [EncryptedData] type.
/// These keys are themselves encrypted using an [EncryptionKeyRecord],
/// which can point to an in-memory key or a remote KMS.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct EncryptionKeyRecord {
  /// The unique encryption key id
  pub id: EncryptionKeyId,
  /// The name of the encryption key. Must be unique.
  pub name: String,
  /// The kind of encryption key.
  /// - Memory
  /// - Disk
  pub kind: EncryptionKeyKind,
  /// For on disk keys (unsafe),
  /// store the base64url encoded key
  pub key: Option<String>,
  /// 32 random bytes base64url encoded
  pub verification: String,
  /// verification bytes encrypted using the master key
  /// and verification nonce.
  pub verification_encrypted: String,
  /// The nonce to use to verify that 'verification_encrypted'
  /// decrypts to 'verification'.
  pub verification_nonce: String,
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

/// The available kinds external of encryption keys.
#[typeshare]
#[derive(
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  Hash,
  Serialize,
  Deserialize,
  Display,
  EnumString,
  AsRefStr,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum EncryptionKeyKind {
  /// Store the encryption key in memory.
  /// These must be initialized via API call after each Cicada Core startup.
  Memory,
  /// UNSAFE DEVELOPMENT OPTION, DO NOT USE IN PRODUCTION.
  /// Load the key from a base64 encoded file on local disk.
  Disk,
}

impl SurrealValue for EncryptionKeyKind {
  fn kind_of() -> surrealdb_types::Kind {
    surrealdb_types::Kind::String
  }

  fn into_value(self) -> surrealdb_types::Value {
    surrealdb_types::Value::String(self.to_string())
  }

  fn from_value(
    value: surrealdb_types::Value,
  ) -> Result<Self, surrealdb_types::Error>
  where
    Self: Sized,
  {
    let surrealdb_types::Value::String(kind) = value else {
      return Err(surrealdb_types::Error::serialization(
        String::from("Value is not String"),
        SerializationError::Deserialization,
      ));
    };
    kind.parse().map_err(|e| {
      surrealdb_types::Error::serialization(
        format!("Invalid EncryptionKeyKind: {e:?}"),
        SerializationError::Deserialization,
      )
    })
  }
}
