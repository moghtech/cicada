use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::entities::{
  EncryptedData, Iso8601Timestamp, encryption_key::EncryptionKeyId,
  node::NodeId, secret::SecretId,
};

/// Checkpoints queryable as a list
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CheckpointListItem {
  /// The unique checkpoint id
  pub id: CheckpointId,
  /// The associated node or secret
  pub target: CheckpointTarget,
  /// The optional name of the checkpoint
  pub name: String,
  /// The optional description for the checkpoint
  pub description: String,
  /// The encryption key used with data
  pub encryption_key: EncryptionKeyId,
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
  /// The associated node or secret
  pub target: CheckpointTarget,
  /// The optional name of the checkpoint
  pub name: String,
  /// The optional description for the checkpoint
  pub description: String,
  /// The master encryption key for the data.
  /// If data is null, it means
  /// the encryption key is not initialized.
  pub encryption_key: EncryptionKeyId,
  /// Data associated with the checkpoint.
  pub data: Option<String>,
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
  /// The associated node or secret
  pub target: CheckpointTarget,
  /// The optional name of the checkpoint
  pub name: String,
  /// The optional description for the checkpoint
  pub description: String,
  /// The master encryption key for this secret, if set.
  /// If this is not null while data is, it means
  /// the encryption key is not initialized.
  pub encryption_key: EncryptionKeyId,
  /// Data associated with the checkpoint.
  pub data: EncryptedData,
  /// Created at as ISO8601 timestamp.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub created_at: Iso8601Timestamp,
  /// Updated at as ISO8601 timestamp.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub updated_at: Iso8601Timestamp,
}

#[typeshare(serialized_as = "string")]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CheckpointId(pub String);

crate::surreal_id!(CheckpointId, "Checkpoint");

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(tag = "type", content = "id")]
pub enum CheckpointTarget {
  Node(NodeId),
  Secret(SecretId),
}

impl CheckpointTarget {
  pub fn associated_data(&self) -> &String {
    match self {
      CheckpointTarget::Node(id) => &id.0,
      CheckpointTarget::Secret(id) => &id.0,
    }
  }
}

impl surrealdb_types::SurrealValue for CheckpointTarget {
  fn kind_of() -> surrealdb_types::Kind {
    surrealdb_types::Kind::Record(vec![])
  }

  fn into_value(self) -> surrealdb_types::Value {
    let record_id = match self {
      CheckpointTarget::Node(id) => id.as_record_id(),
      CheckpointTarget::Secret(id) => id.as_record_id(),
    };
    surrealdb_types::Value::RecordId(record_id)
  }

  fn from_value(
    value: surrealdb_types::Value,
  ) -> Result<Self, surrealdb_types::Error>
  where
    Self: Sized,
  {
    let surrealdb_types::Value::RecordId(record_id) = value else {
      return Err(surrealdb_types::Error::serialization(
        String::from("Value is not RecordId"),
        surrealdb_types::SerializationError::Deserialization,
      ));
    };
    let surrealdb_types::RecordIdKey::String(id) = record_id.key
    else {
      return Err(surrealdb_types::Error::serialization(
        String::from("RecordIdKey is not String"),
        surrealdb_types::SerializationError::Deserialization,
      ));
    };
    match record_id.table.as_str() {
      "Node" => Ok(Self::Node(NodeId(id))),
      "Secret" => Ok(Self::Secret(SecretId(id))),
      other => Err(surrealdb_types::Error::serialization(
        format!("Unknown table: {other}"),
        surrealdb_types::SerializationError::Deserialization,
      )),
    }
  }
}
