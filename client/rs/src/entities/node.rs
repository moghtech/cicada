use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::entities::U64;

// #[typeshare]
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct NodeEntity {}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
pub struct NodeRecord {
  // /// The database entry ID
  // pub id: RecordId,
  /// The unique inode number
  pub ino: U64,
  /// The parent node ID
  pub parent: U64,
  /// The name of the node
  pub name: String,
  /// The kind of node.
  /// - Folder,
  /// - File,
  pub kind: NodeKind,
  /// Data associated with the node.
  /// For files, this contains the file contents.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data: Option<String>,
}

/// Nodes can be either folders or files.
#[typeshare]
#[derive(
  Debug, Clone, Copy, Serialize, Deserialize, SurrealValue,
)]
#[surreal(untagged)]
pub enum NodeKind {
  Folder,
  File,
}
