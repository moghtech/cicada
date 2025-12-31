use derive_empty_traits::EmptyTraits;
use resolver_api::Resolve;
use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::{
  api::write::CicadaWriteRequest,
  entities::{
    U64,
    filesystem::FilesystemId,
    node::{NodeId, NodeKind, NodeRecord},
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
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(CreateNodeResponse)]
#[error(serror::Error)]
pub struct CreateNode {
  /// The filesystem ID
  pub filesystem: Option<FilesystemId>,
  /// parent inode number.
  /// Default: 1 (the root node).
  #[cfg_attr(feature = "openapi", schema(minimum = 1, default = 1))]
  pub parent: Option<U64>,
  /// The name of the node
  pub name: String,
  /// The kind of node.
  /// - Folder
  /// - File
  ///
  /// Default: **Folder**
  #[cfg_attr(feature = "openapi", schema(default = "Folder"))]
  pub kind: Option<NodeKind>,
  /// Data associated with the node.
  /// For files, this contains the file contents.
  pub data: Option<String>,
}

/// Response for [CreateNode].
#[typeshare]
pub type CreateNodeResponse = NodeRecord;

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
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(UpdateNodeResponse)]
#[error(serror::Error)]
pub struct UpdateNode {
  /// The node id
  pub id: NodeId,
  /// parent inode number.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub parent: Option<U64>,
  /// The name of the node
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  /// Data associated with the node.
  /// For files, this contains the file contents.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data: Option<String>,
}

/// Response for [UpdateNode].
#[typeshare]
pub type UpdateNodeResponse = NodeRecord;
