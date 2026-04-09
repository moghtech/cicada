use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::entities::{
  InterpolationMode, Iso8601Timestamp,
  encryption_key::EncryptionKeyId,
};

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct FilesystemRecord {
  /// The unique filesystem id
  pub id: FilesystemId,
  /// The name of the filesystem. Must be unique.
  pub name: String,
  /// The filesystem default encryption key.
  pub encryption_key: Option<EncryptionKeyId>,
  /// The default interpolation mode for the filesystem
  #[surreal(default)]
  pub interpolation: InterpolationMode,
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
pub struct FilesystemId(pub String);

crate::surreal_id!(FilesystemId, "Filesystem");
