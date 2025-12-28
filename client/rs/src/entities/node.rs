use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::entities::U64;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
pub struct NodeListItem {
  /// Filesystem ID
  pub filesystem: String,
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
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
pub struct NodeRecord {
  /// Filesystem ID
  pub filesystem: String,
  /// The unique inode number
  pub ino: U64,
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

impl NodeRecord {
  pub fn root(filesystem: String) -> NodeRecord {
    NodeRecord {
      filesystem,
      ino: 1,
      parent: 0,
      name: String::from("root"),
      kind: NodeKind::Folder,
      data: None,
    }
  }
}

/// Nodes can be either folders or files.
#[typeshare]
#[derive(
  Debug, Clone, Copy, Default, Serialize, Deserialize, SurrealValue,
)]
#[surreal(untagged)]
pub enum NodeKind {
  #[default]
  Folder,
  File,
}
