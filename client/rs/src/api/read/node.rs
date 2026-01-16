use derive_empty_traits::EmptyTraits;
use resolver_api::Resolve;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
  api::read::CicadaReadRequest,
  entities::{
    U64,
    filesystem::FilesystemId,
    node::{NodeEntity, NodeId, NodeListItem},
  },
};

//

/// List filesystem nodes. Response: [ListNodesResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, Resolve, EmptyTraits,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaReadRequest)]
#[response(ListNodesResponse)]
#[error(mogh_error::Error)]
pub struct ListNodes {
  /// Filesystem id
  pub filesystem: Option<FilesystemId>,
  /// parent inode number.
  #[cfg_attr(feature = "utoipa", schema(minimum = 1))]
  pub parent: Option<U64>,
}

/// Response for [ListNodes].
#[typeshare]
pub type ListNodesResponse = Vec<NodeListItem>;

//

/// Get a node. Response: [NodeEntity].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, Resolve, EmptyTraits,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaReadRequest)]
#[response(GetNodeResponse)]
#[error(mogh_error::Error)]
pub struct GetNode {
  /// The node id
  pub id: NodeId,
}

/// Response for [GetNode].
#[typeshare]
pub type GetNodeResponse = NodeEntity;

//

/// Find a node. Response: [NodeEntity].
///
/// Query using either:
/// - inode number (ino)
/// - name (parent inode number defaults to 1)
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, Resolve, EmptyTraits,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaReadRequest)]
#[response(FindNodeResponse)]
#[error(mogh_error::Error)]
pub struct FindNode {
  /// Filesystem id
  pub filesystem: FilesystemId,
  /// The node inode number
  #[cfg_attr(feature = "utoipa", schema(minimum = "1"))]
  pub inode: Option<U64>,
  /// The node parent inode number.
  /// Default: 1 (the root node).
  #[cfg_attr(feature = "utoipa", schema(minimum = "1"))]
  pub parent: Option<U64>,
  /// file name
  pub name: Option<String>,
}

impl FindNode {
  pub fn with_inode(
    filesystem: FilesystemId,
    inode: u64,
  ) -> FindNode {
    FindNode {
      filesystem,
      inode: inode.into(),
      parent: None,
      name: None,
    }
  }

  pub fn with_parent_name(
    filesystem: FilesystemId,
    parent: u64,
    name: impl Into<String>,
  ) -> FindNode {
    FindNode {
      filesystem,
      parent: parent.into(),
      name: name.into().into(),
      inode: None,
    }
  }
}

/// Response for [FindNode].
#[typeshare]
pub type FindNodeResponse = NodeEntity;
