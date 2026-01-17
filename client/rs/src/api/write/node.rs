use derive_empty_traits::EmptyTraits;
use resolver_api::Resolve;
use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::{
  api::write::CicadaWriteRequest,
  entities::{
    U64,
    encryption_key::EncryptionKeyId,
    filesystem::FilesystemId,
    node::{NodeEntity, NodeId, NodeKind},
  },
};

//

/// Create filesystem node. Response: [CreateNodeResponse].
#[typeshare]
#[derive(
  Debug,
  Clone,
  Serialize,
  Deserialize,
  SurrealValue,
  Resolve,
  EmptyTraits,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(CreateNodeResponse)]
#[error(mogh_error::Error)]
pub struct CreateNode {
  /// The filesystem ID
  pub filesystem: Option<FilesystemId>,
  /// parent inode number.
  /// Default: 1 (the root node).
  #[cfg_attr(feature = "utoipa", schema(minimum = 1, default = 1))]
  pub parent: Option<U64>,
  /// The name of the node
  pub name: String,
  /// The kind of node.
  /// - Folder
  /// - File
  ///
  /// Default: **Folder**
  #[cfg_attr(feature = "utoipa", schema(default = "Folder"))]
  pub kind: Option<NodeKind>,
  /// Data associated with the node.
  /// For files, this contains the file contents.
  pub data: Option<String>,
  /// Choose a specific encryption key.
  /// Otherwise uses the current filesystem default,
  /// followed by the current global default.
  pub encryption_key: Option<EncryptionKeyId>,
}

/// Response for [CreateNode].
#[typeshare]
pub type CreateNodeResponse = NodeEntity;

//

/// Update a filesystem node. Response: [UpdateNodeResponse].
#[typeshare]
#[derive(
  Debug,
  Clone,
  Serialize,
  Deserialize,
  SurrealValue,
  Resolve,
  EmptyTraits,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(UpdateNodeResponse)]
#[error(mogh_error::Error)]
pub struct UpdateNode {
  /// The node id
  pub id: NodeId,
  /// parent inode number.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub parent: Option<U64>,
  /// The name of the node
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
}

/// Response for [UpdateNode].
#[typeshare]
pub type UpdateNodeResponse = NodeEntity;

//

/// Update a filesystem node's encrypted data. Response: [UpdateNodeDataResponse].
#[typeshare]
#[derive(
  Debug,
  Clone,
  Serialize,
  Deserialize,
  SurrealValue,
  Resolve,
  EmptyTraits,
)]
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
}

/// Response for [UpdateNodeData].
#[typeshare]
pub type UpdateNodeDataResponse = NodeEntity;

//

/// Update a filesystem node's encryption key. Response: [UpdateNodeEncryptionKeyResponse].
#[typeshare]
#[derive(
  Debug,
  Clone,
  Serialize,
  Deserialize,
  SurrealValue,
  Resolve,
  EmptyTraits,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(UpdateNodeEncryptionKeyResponse)]
#[error(mogh_error::Error)]
pub struct UpdateNodeEncryptionKey {
  /// The node id
  pub id: NodeId,
  /// Update the encryption key used as master in the envelope encryption.
  pub encryption_key: EncryptionKeyId,
}

/// Response for [UpdateNodeEncryptionKey].
#[typeshare]
pub type UpdateNodeEncryptionKeyResponse = NodeEntity;

//

/// Rotate a filesystem node's envelope encryption key. Response: [RotateNodeEnvelopeKeyResponse].
#[typeshare]
#[derive(
  Debug,
  Clone,
  Serialize,
  Deserialize,
  SurrealValue,
  Resolve,
  EmptyTraits,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(RotateNodeEnvelopeKeyResponse)]
#[error(mogh_error::Error)]
pub struct RotateNodeEnvelopeKey {
  /// The node id
  pub id: NodeId,
}

/// Response for [RotateNodeEnvelopeKey].
#[typeshare]
pub type RotateNodeEnvelopeKeyResponse = NodeEntity;

//

/// Delete a filesystem node. Response: [DeleteNodeResponse].
///
/// WARNING: If the node is a folder and `move_children`
/// is not passed, all children nodes will be recursively deleted.
#[typeshare]
#[derive(
  Debug,
  Clone,
  Serialize,
  Deserialize,
  SurrealValue,
  Resolve,
  EmptyTraits,
)]
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
}

/// Response for [DeleteNode].
#[typeshare]
pub type DeleteNodeResponse = Vec<NodeEntity>;

//

/// Batch delete files / folders. Response: [BatchDeleteNodesResponse].
///
/// Note. Not compatible with 'move_children'.
#[typeshare]
#[derive(
  Debug,
  Clone,
  Serialize,
  Deserialize,
  SurrealValue,
  Resolve,
  EmptyTraits,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(BatchDeleteNodesResponse)]
#[error(mogh_error::Error)]
pub struct BatchDeleteNodes {
  /// The onboarding_key ID
  pub ids: Vec<NodeId>,
}

/// Response for [BatchDeleteNodes].
#[typeshare]
pub type BatchDeleteNodesResponse = Vec<NodeEntity>;
