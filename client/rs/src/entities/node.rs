use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::entities::{
  CheckpointingMode, EncryptedData, InterpolationMode,
  Iso8601Timestamp, U64, encryption_key::EncryptionKeyId,
  filesystem::FilesystemId,
};

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NodeListItem {
  /// The unique node id
  pub id: NodeId,
  /// Filesystem ID
  pub filesystem: FilesystemId,
  /// The inode number
  pub inode: U64,
  /// The parent node ID
  pub parent: U64,
  /// The name of the node
  pub name: String,
  /// The file permission integer.
  /// Usually represented as octet like 0o644.
  /// If not provided, will use defaults:
  /// - Folder: 0o755
  /// - File: 0o644
  pub perm: Option<u16>,
  /// The kind of node.
  /// - Folder,
  /// - File,
  pub kind: NodeKind,
  /// The file checkpointing mode
  /// - `"Inherit"` (default)
  /// - `"Enabled"`
  /// - `"Disabled"`
  pub checkpointing: CheckpointingMode,
  /// The interpolation mode
  pub interpolation: InterpolationMode,
  /// The encryption key used with data
  pub encryption_key: Option<EncryptionKeyId>,
  /// Created at as ISO8601 timestamp.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub created_at: Iso8601Timestamp,
  /// Updated at as ISO8601 timestamp.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub updated_at: Iso8601Timestamp,
}

/// Nodes over the API, with unencrypted data
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NodeEntity {
  /// The unique node id
  pub id: NodeId,
  /// Filesystem ID
  pub filesystem: FilesystemId,
  /// The inode number
  pub inode: U64,
  /// The parent inode number
  pub parent: U64,
  /// The name of the node
  pub name: String,
  /// The file permission integer.
  /// Usually represented as octet like 0o644.
  /// If not provided, will use defaults:
  /// - Folder: 0o755
  /// - File: 0o644
  pub perm: Option<u16>,
  /// The kind of node.
  /// - Folder,
  /// - File,
  pub kind: NodeKind,
  /// The file checkpointing mode
  /// - `"Inherit"` (default)
  /// - `"Enabled"`
  /// - `"Disabled"`
  pub checkpointing: CheckpointingMode,
  /// The interpolation mode
  pub interpolation: InterpolationMode,
  /// Data associated with the node.
  /// For files, this contains the file contents.
  pub data: Option<String>,
  /// The encryption key used with data
  pub encryption_key: Option<EncryptionKeyId>,
  /// Whether encryption key is not initialized
  pub missing_key: bool,
  /// Created at as ISO8601 timestamp.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub created_at: Iso8601Timestamp,
  /// Updated at as ISO8601 timestamp.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub updated_at: Iso8601Timestamp,
}

/// Nodes stored on the database, with encrypted data
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NodeRecord {
  /// The unique node id
  pub id: NodeId,
  /// Filesystem ID
  pub filesystem: FilesystemId,
  /// The inode number
  pub inode: U64,
  /// The parent inode number
  pub parent: U64,
  /// The name of the node
  pub name: String,
  /// The file permission integer.
  /// Usually represented as octet like 0o644.
  /// If not provided, will use defaults:
  /// - Folder: 0o755
  /// - File: 0o644
  pub perm: Option<u16>,
  /// The kind of node.
  /// - Folder,
  /// - File,
  pub kind: NodeKind,
  /// The file checkpointing mode
  /// - `"Inherit"` (default)
  /// - `"Enabled"`
  /// - `"Disabled"`
  pub checkpointing: CheckpointingMode,
  /// The interpolation mode
  pub interpolation: InterpolationMode,
  /// Data associated with the node.
  /// For files, this contains the file contents.
  pub data: Option<EncryptedData>,
  /// Created at as ISO8601 timestamp.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub created_at: Iso8601Timestamp,
  /// Updated at as ISO8601 timestamp.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub updated_at: Iso8601Timestamp,
}

impl NodeRecord {
  pub fn into_entity(
    self,
    data: Option<String>,
    encryption_key: Option<EncryptionKeyId>,
    missing_key: bool,
  ) -> NodeEntity {
    NodeEntity {
      id: self.id,
      filesystem: self.filesystem,
      inode: self.inode,
      parent: self.parent,
      name: self.name,
      perm: self.perm,
      kind: self.kind,
      checkpointing: self.checkpointing,
      interpolation: self.interpolation,
      created_at: self.created_at,
      updated_at: self.updated_at,
      data,
      encryption_key,
      missing_key,
    }
  }
}

/// Nodes can be either folders or files.
#[typeshare]
#[derive(
  Debug, Clone, Copy, Default, Serialize, Deserialize, SurrealValue,
)]
#[surreal(crate = "surrealdb_types")]
#[surreal(untagged)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum NodeKind {
  #[default]
  Folder,
  File,
}

#[typeshare(serialized_as = "string")]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NodeId(pub String);

crate::surreal_id!(NodeId, "Node");
