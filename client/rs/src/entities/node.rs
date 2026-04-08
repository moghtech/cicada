use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::entities::{
  EncryptedData, InterpolationMode, Iso8601Timestamp, U64,
  encryption_key::EncryptionKeyId, filesystem::FilesystemId,
};

fn default_interpolation() -> InterpolationMode {
  InterpolationMode::Inherit
}

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
  /// The interpolation mode
  #[serde(default = "default_interpolation")]
  pub interpolation: InterpolationMode,
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
  #[serde(default)]
  pub kind: NodeKind,
  /// The interpolation mode
  #[serde(default = "default_interpolation")]
  pub interpolation: InterpolationMode,
  /// Data associated with the node.
  /// For files, this contains the file contents.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data: Option<String>,
  /// If the data could not be decrypted
  /// due to missing encryption key, give the missing ID
  /// for the user to know to initialize.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub missing_key: Option<EncryptionKeyId>,
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
  #[serde(default)]
  pub kind: NodeKind,
  /// The interpolation mode
  #[serde(default = "default_interpolation")]
  pub interpolation: InterpolationMode,
  /// Data associated with the node.
  /// For files, this contains the file contents.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data: Option<EncryptedData>,
  /// Created at as ISO8601 timestamp.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub created_at: Iso8601Timestamp,
  /// Updated at as ISO8601 timestamp.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub updated_at: Iso8601Timestamp,
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
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NodeId(pub String);

crate::surreal_id!(NodeId, "Node");
