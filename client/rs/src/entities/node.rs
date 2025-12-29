use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use surrealdb_types::{RecordIdKey, SurrealValue};
use typeshare::typeshare;
use utoipa::ToSchema;

use crate::entities::{U64, filesystem::FilesystemId};

#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, ToSchema,
)]
pub struct NodeListItem {
  /// The unique node id
  pub id: NodeId,
  /// Filesystem ID
  pub filesystem: FilesystemId,
  /// The parent node ID
  pub parent: U64,
  /// The name of the node
  pub name: String,
  /// The kind of node.
  /// - Folder,
  /// - File,
  pub kind: NodeKind,
}

#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, ToSchema,
)]
pub struct NodeRecord {
  /// The unique node id
  pub id: NodeId,
  /// Filesystem ID
  pub filesystem: FilesystemId,
  /// The parent node ID
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
}

/// Nodes can be either folders or files.
#[typeshare]
#[derive(
  Debug,
  Clone,
  Copy,
  Default,
  Serialize,
  Deserialize,
  SurrealValue,
  ToSchema,
)]
#[surreal(untagged)]
pub enum NodeKind {
  #[default]
  Folder,
  File,
}

#[typeshare(serialized_as = "number")]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NodeId(pub U64);

impl SurrealValue for NodeId {
  fn kind_of() -> surrealdb_types::Kind {
    surrealdb_types::Kind::Record(vec![])
  }

  fn into_value(self) -> surrealdb_types::Value {
    surrealdb_types::Value::RecordId(surrealdb_types::RecordId::new(
      "Node",
      self.0 as i64,
    ))
  }

  fn from_value(value: surrealdb_types::Value) -> anyhow::Result<Self>
  where
    Self: Sized,
  {
    let surrealdb_types::Value::RecordId(id) = value else {
      return Err(anyhow!("Value is not RecordId"));
    };
    let RecordIdKey::Number(id) = id.key else {
      return Err(anyhow!("RecordIdKey is not Number"));
    };
    Ok(Self(id as u64))
  }
}
