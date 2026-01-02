use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use surrealdb_types::{RecordId, RecordIdKey, SurrealValue};
use typeshare::typeshare;

use crate::entities::{
  Iso8601Timestamp, U64, filesystem::FilesystemId,
};

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct NodeListItem {
  /// The unique node id
  pub id: NodeId,
  /// Filesystem ID
  pub filesystem: FilesystemId,
  /// The inode number
  pub ino: U64,
  /// The parent node ID
  pub parent: U64,
  /// The name of the node
  pub name: String,
  /// The kind of node.
  /// - Folder,
  /// - File,
  pub kind: NodeKind,
  /// Created at as ISO8601 timestamp.
  #[cfg_attr(feature = "openapi", schema(value_type = String))]
  pub created_at: Iso8601Timestamp,
  /// Updated at as ISO8601 timestamp.
  #[cfg_attr(feature = "openapi", schema(value_type = String))]
  pub updated_at: Iso8601Timestamp,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct NodeRecord {
  /// The unique node id
  pub id: NodeId,
  /// Filesystem ID
  pub filesystem: FilesystemId,
  /// The inode number
  pub ino: U64,
  /// The parent inode number
  pub parent: U64,
  /// The name of the node
  pub name: String,
  /// The kind of node.
  /// - Folder,
  /// - File,
  #[serde(default)]
  pub kind: NodeKind,
  /// Data associated with the node.
  /// For files, this contains the file contents.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data: Option<String>,
  /// Created at as ISO8601 timestamp.
  #[cfg_attr(feature = "openapi", schema(value_type = String))]
  pub created_at: Iso8601Timestamp,
  /// Updated at as ISO8601 timestamp.
  #[cfg_attr(feature = "openapi", schema(value_type = String))]
  pub updated_at: Iso8601Timestamp,
}

/// Nodes can be either folders or files.
#[typeshare]
#[derive(
  Debug, Clone, Copy, Default, Serialize, Deserialize, SurrealValue,
)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[surreal(untagged)]
pub enum NodeKind {
  #[default]
  Folder,
  File,
}

#[typeshare(serialized_as = "string")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct NodeId(pub String);

impl NodeId {
  pub fn as_record_id(&self) -> RecordId {
    RecordId::new("Node", self.0.as_str())
  }
}

impl SurrealValue for NodeId {
  fn kind_of() -> surrealdb_types::Kind {
    surrealdb_types::Kind::Record(vec![])
  }

  fn into_value(self) -> surrealdb_types::Value {
    surrealdb_types::Value::RecordId(surrealdb_types::RecordId::new(
      "Node", self.0,
    ))
  }

  fn from_value(value: surrealdb_types::Value) -> anyhow::Result<Self>
  where
    Self: Sized,
  {
    let surrealdb_types::Value::RecordId(id) = value else {
      return Err(anyhow!("Value is not RecordId"));
    };
    let RecordIdKey::String(id) = id.key else {
      return Err(anyhow!("RecordIdKey is not String"));
    };
    Ok(Self(id))
  }
}
