use mogh_resolver::Resolve;
use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::{
  api::write::CicadaWriteRequest,
  entities::{
    CheckpointingMode, InterpolationMode, U64,
    encryption_key::EncryptionKeyId,
    filesystem::FilesystemId,
    node::{NodeEntity, NodeId, NodeKind},
  },
};

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/CreateNode",
  description = "Create a new node",
  request_body(content = CreateNode),
  responses(
    (status = 200, description = "The created node", body = CreateNodeResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn create_node() {}

/// Create filesystem node. Response: [CreateNodeResponse].
#[typeshare]
#[derive(
  Debug, Clone, Default, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(CreateNodeResponse)]
#[error(mogh_error::Error)]
pub struct CreateNode {
  /// The filesystem ID
  pub filesystem: FilesystemId,
  /// parent inode number.
  /// Default: 1 (the root node).
  #[cfg_attr(feature = "utoipa", schema(minimum = 1, default = 1))]
  #[serde(default = "default_parent")]
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
  /// - Folder
  /// - File
  ///
  /// Default: **Folder**
  #[cfg_attr(feature = "utoipa", schema(default = "Folder"))]
  #[serde(default)]
  pub kind: NodeKind,
  /// The file checkpointing mode
  /// - `"Inherit"` (default)
  /// - `"Enabled"`
  /// - `"Disabled"`
  pub checkpointing: Option<CheckpointingMode>,
  /// The interpolation mode (only for files)
  /// - `"Inherit"` (inherit from filesystem option) (default)
  /// - `"Brackets"` (`[[SECRET]]`)
  /// - `"CurlyBrackets"` (`{{SECRET}}`)
  /// - `"EnvVar"` (`${SECRET}`)
  /// - `"Disabled"` (Interpolation disabled for this file)
  pub interpolation: Option<InterpolationMode>,
  /// Data associated with the node.
  /// For files, this contains the file contents.
  pub data: Option<String>,
  /// Choose a specific encryption key.
  /// Otherwise uses the current filesystem default,
  /// followed by the current global default.
  pub encryption_key: Option<EncryptionKeyId>,
  /// Whether to store the contents as a restorable checkpoint.
  /// This will always be done if checkpointing is enabled on the node.
  pub checkpoint: Option<bool>,
  /// Save the checkpoint with this name.
  pub checkpoint_name: Option<String>,
  /// Save the checkpoint with this description
  pub checkpoint_description: Option<String>,
  /// Whether to interpolate secrets into returned file contents
  #[serde(default)]
  pub interpolated: bool,
}

fn default_parent() -> u64 {
  1
}

/// Response for [CreateNode].
#[typeshare]
pub type CreateNodeResponse = NodeEntity;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/UpdateNode",
  description = "Update a node",
  request_body(content = UpdateNode),
  responses(
    (status = 200, description = "The updated node", body = UpdateNodeResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn update_node() {}

/// Update a filesystem node. Response: [UpdateNodeResponse].
#[typeshare]
#[derive(
  Debug, Clone, Default, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(UpdateNodeResponse)]
#[error(mogh_error::Error)]
pub struct UpdateNode {
  /// The node id
  pub id: NodeId,
  /// The filesystem ID
  pub filesystem: Option<FilesystemId>,
  /// parent inode number.
  pub parent: Option<U64>,
  /// The name of the node
  pub name: Option<String>,
  /// The file permission integer.
  /// Usually represented as octet like 0o644.
  /// If not provided, will use defaults:
  /// - Folder: 0o755
  /// - File: 0o644
  pub perm: Option<u16>,
  /// The file checkpointing mode
  /// - `"Inherit"` (default)
  /// - `"Enabled"`
  /// - `"Disabled"`
  pub checkpointing: Option<CheckpointingMode>,
  /// The interpolation mode (only for files)
  /// - `"Inherit"` (inherit from filesystem option) (default)
  /// - `"Brackets"` (`[[SECRET]]`)
  /// - `"CurlyBrackets"` (`{{SECRET}}`)
  /// - `"EnvVar"` (`${SECRET}`)
  /// - `"Disabled"` (Interpolation disabled for this file)
  pub interpolation: Option<InterpolationMode>,
  /// Whether to interpolate secrets into returned file contents
  pub interpolated: Option<bool>,
}

/// Response for [UpdateNode].
#[typeshare]
pub type UpdateNodeResponse = NodeEntity;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/UpdateNodeData",
  description = "Update a node's data",
  request_body(content = UpdateNode),
  responses(
    (status = 200, description = "The updated node", body = UpdateNodeDataResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn update_node_data() {}

/// Update a filesystem node's encrypted data. Response: [UpdateNodeDataResponse].
#[typeshare]
#[derive(
  Debug, Clone, Default, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(UpdateNodeResponse)]
#[error(mogh_error::Error)]
pub struct UpdateNodeData {
  /// The node id
  pub id: NodeId,
  /// The node data
  pub data: String,
  /// Optionally update the encryption key used as master in the envelope encryption.
  pub encryption_key: Option<EncryptionKeyId>,
  /// Whether to store the contents as a restorable checkpoint.
  /// This will always be done if checkpointing is enabled on the node.
  pub checkpoint: Option<bool>,
  /// Save the checkpoint with this name.
  pub checkpoint_name: Option<String>,
  /// Save the checkpoint with this description
  pub checkpoint_description: Option<String>,
  /// Whether to interpolate secrets into returned file contents
  #[serde(default)]
  pub interpolated: bool,
}

/// Response for [UpdateNodeData].
#[typeshare]
pub type UpdateNodeDataResponse = NodeEntity;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/UpdateNodeEncryptionKey",
  description = "Update a node's data",
  request_body(content = UpdateNode),
  responses(
    (status = 200, description = "The updated node", body = UpdateNodeEncryptionKeyResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn update_node_encryption_key() {}

/// Update a filesystem node's encryption key. Response: [UpdateNodeEncryptionKeyResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(UpdateNodeEncryptionKeyResponse)]
#[error(mogh_error::Error)]
pub struct UpdateNodeEncryptionKey {
  /// The node id
  pub id: NodeId,
  /// Update the encryption key used as master in the envelope encryption.
  pub encryption_key: EncryptionKeyId,
  /// Whether to interpolate secrets into returned file contents
  #[serde(default)]
  pub interpolated: bool,
}

/// Response for [UpdateNodeEncryptionKey].
#[typeshare]
pub type UpdateNodeEncryptionKeyResponse = NodeEntity;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/RotateNodeEnvelopeKey",
  description = "Update a node's data",
  request_body(content = UpdateNode),
  responses(
    (status = 200, description = "The updated node", body = RotateNodeEnvelopeKeyResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn rotate_node_envelope_key() {}

/// Rotate a filesystem node's envelope encryption key. Response: [RotateNodeEnvelopeKeyResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(RotateNodeEnvelopeKeyResponse)]
#[error(mogh_error::Error)]
pub struct RotateNodeEnvelopeKey {
  /// The node id
  pub id: NodeId,
  /// Whether to interpolate secrets into returned file contents
  #[serde(default)]
  pub interpolated: bool,
}

/// Response for [RotateNodeEnvelopeKey].
#[typeshare]
pub type RotateNodeEnvelopeKeyResponse = NodeEntity;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/DeleteNode",
  description = "Delete a node",
  request_body(content = DeleteNode),
  responses(
    (status = 200, description = "The deleted nodes", body = DeleteNodeResponse),
    (status = 404, description = "Node not found", body = mogh_error::Serror),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn delete_node() {}

/// Delete a filesystem node. Response: [DeleteNodeResponse].
///
/// WARNING: If the node is a folder and `move_children`
/// is not passed, all children nodes will be recursively deleted.
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(DeleteNodeResponse)]
#[error(mogh_error::Error)]
pub struct DeleteNode {
  /// The node id
  pub id: NodeId,
  /// Move the children of this node to another parent.
  /// Otherwise, all children will be recursively deleted.
  pub move_children: Option<U64>,
  /// Whether to interpolate secrets into returned file contents
  #[serde(default)]
  pub interpolated: bool,
}

/// Response for [DeleteNode].
#[typeshare]
pub type DeleteNodeResponse = Vec<NodeEntity>;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/BatchDeleteNodes",
  description = "Batch delete many files / folders recursively.",
  request_body(content = BatchDeleteNodes),
  responses(
    (status = 200, description = "The deleted files / folders", body = BatchDeleteNodesResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn batch_delete_nodes() {}

/// Batch delete files / folders. Response: [BatchDeleteNodesResponse].
///
/// Note. Not compatible with 'move_children'.
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(BatchDeleteNodesResponse)]
#[error(mogh_error::Error)]
pub struct BatchDeleteNodes {
  /// The onboarding_key ID
  pub ids: Vec<NodeId>,
  /// Whether to interpolate secrets into returned file contents
  #[serde(default)]
  pub interpolated: bool,
}

/// Response for [BatchDeleteNodes].
#[typeshare]
pub type BatchDeleteNodesResponse = Vec<NodeEntity>;
