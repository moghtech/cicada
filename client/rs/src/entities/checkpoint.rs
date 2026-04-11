use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::entities::{
  EncryptedData, Iso8601Timestamp, encryption_key::EncryptionKeyId,
  node::NodeId,
};

/// Checkpoints queryable as a list
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CheckpointListItem {
  /// The unique checkpoint id
  pub id: CheckpointId,
  /// The associated node
  pub node: NodeId,
  /// The optional name of the checkpoint
  pub name: String,
  /// The optional description for the checkpoint
  pub description: String,
  /// The encryption key used with data
  pub encryption_key: Option<EncryptionKeyId>,
  /// Created at as ISO8601 timestamp.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub created_at: Iso8601Timestamp,
  /// Updated at as ISO8601 timestamp.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub updated_at: Iso8601Timestamp,
}

/// Checkpoints over the API, with unencrypted data
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CheckpointEntity {
  /// The unique checkpoint id
  pub id: CheckpointId,
  /// The associated node
  pub node: NodeId,
  /// The optional name of the checkpoint
  pub name: String,
  /// The optional description for the checkpoint
  pub description: String,
  /// Data associated with the checkpoint.
  pub data: Option<String>,
  /// The encryption key used with data
  pub encryption_key: Option<EncryptionKeyId>,
  /// Created at as ISO8601 timestamp.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub created_at: Iso8601Timestamp,
  /// Updated at as ISO8601 timestamp.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub updated_at: Iso8601Timestamp,
}

/// Checkpoints stored on the database, with encrypted data
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CheckpointRecord {
  /// The unique checkpoint id
  pub id: CheckpointId,
  /// The associated node
  pub node: NodeId,
  /// The optional name of the checkpoint
  pub name: String,
  /// The optional description for the checkpoint
  pub description: String,
  /// Data associated with the checkpoint.
  pub data: Option<EncryptedData>,
  /// Created at as ISO8601 timestamp.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub created_at: Iso8601Timestamp,
  /// Updated at as ISO8601 timestamp.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub updated_at: Iso8601Timestamp,
}

impl CheckpointRecord {
  pub fn into_entity(
    self,
    data: Option<String>,
    encryption_key: Option<EncryptionKeyId>,
  ) -> CheckpointEntity {
    CheckpointEntity {
      id: self.id,
      node: self.node,
      name: self.name,
      description: self.description,
      created_at: self.created_at,
      updated_at: self.updated_at,
      data,
      encryption_key,
    }
  }
}

#[typeshare(serialized_as = "string")]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CheckpointId(pub String);

crate::surreal_id!(CheckpointId, "Checkpoint");
