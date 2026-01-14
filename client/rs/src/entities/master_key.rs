use serde::{Deserialize, Serialize};
use surrealdb_types::{RecordId, RecordIdKey, SurrealValue};
use typeshare::typeshare;

use crate::entities::Iso8601Timestamp;

/// Record fields are encrypted using encryption keys stored
/// in EncryptedKeyRecord. These keys themselves must be encrypted using
/// a master key.
///
/// Production master keys can be in memory (initialized via API call on startup),
/// or point to a remote KMS.
///
/// This pattern allows both record keys and master keys to be rotated.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct MasterKeyRecord {
  /// The unique master key id
  pub id: MasterKeyId,
  /// The name of the master key. Must be unique.
  pub name: String,
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
pub struct MasterKeyId(pub String);

crate::surreal_id!(MasterKeyId, "MasterKey");
