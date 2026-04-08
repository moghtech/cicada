use std::path::PathBuf;

use mogh_resolver::Resolve;
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

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/read/ListNodes",
  description = "List available folders and files for a filesystem.",
  request_body(content = ListNodes),
  responses(
    (status = 200, description = "List of filesystem nodes", body = ListNodesResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn list_nodes() {}

/// List filesystem nodes. Response: [ListNodesResponse].
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
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

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/read/GetNode",
  description = "Get a folder or file by id",
  request_body(content = GetNode),
  responses(
    (status = 200, description = "The filesystem node", body = GetNodeResponse),
    (status = 404, description = "Failed to find node with given id", body = mogh_error::Serror),
    (status = 500, description = "Request failed", body = mogh_error::Serror),
  ),
)]
pub fn get_node() {}

/// Get a node. Response: [NodeEntity].
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaReadRequest)]
#[response(GetNodeResponse)]
#[error(mogh_error::Error)]
pub struct GetNode {
  /// The node id
  pub id: NodeId,
  /// Whether to interpolate secrets into file contents
  #[serde(default)]
  pub interpolated: bool,
}

/// Response for [GetNode].
#[typeshare]
pub type GetNodeResponse = NodeEntity;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/read/FindNode",
  description = "Find a node by filesystem + inode OR filesystem + parent inode + name",
  request_body(content = FindNode),
  responses(
    (status = 200, description = "The filesystem node", body = FindNodeResponse),
    (status = 404, description = "Failed to find node with given parameters", body = mogh_error::Serror),
    (status = 500, description = "Request failed", body = mogh_error::Serror),
  ),
)]
pub fn find_node() {}

/// Find a node. Response: [NodeEntity].
///
/// Query using either:
/// - inode number (ino)
/// - name (parent inode number defaults to 1)
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
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
  /// Whether to interpolate secrets into file contents
  #[serde(default)]
  pub interpolated: bool,
}

impl FindNode {
  pub fn with_inode(
    filesystem: FilesystemId,
    inode: u64,
    interpolated: bool,
  ) -> FindNode {
    FindNode {
      filesystem,
      inode: inode.into(),
      parent: None,
      name: None,
      interpolated,
    }
  }

  pub fn with_parent_name(
    filesystem: FilesystemId,
    parent: u64,
    name: impl Into<String>,
    interpolated: bool,
  ) -> FindNode {
    FindNode {
      filesystem,
      parent: parent.into(),
      name: name.into().into(),
      inode: None,
      interpolated,
    }
  }
}

/// Response for [FindNode].
#[typeshare]
pub type FindNodeResponse = NodeEntity;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/read/FindNodeWithPath",
  description = "Get a folder or file by id",
  request_body(content = FindNodeWithPath),
  responses(
    (status = 200, description = "The filesystem node", body = FindNodeWithPathResponse),
    (status = 404, description = "Failed to find node with given id", body = mogh_error::Serror),
    (status = 500, description = "Request failed", body = mogh_error::Serror),
  ),
)]
pub fn find_node_with_path() {}

/// Find a node using the path. Response: [NodeEntity].
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaReadRequest)]
#[response(FindNodeWithPathResponse)]
#[error(mogh_error::Error)]
pub struct FindNodeWithPath {
  /// Filesystem id
  pub filesystem: FilesystemId,
  /// The path to the node. `/path/to/node`
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub path: PathBuf,
  /// Whether to interpolate secrets into file contents
  #[serde(default)]
  pub interpolated: bool,
}

/// Response for [FindNodeWithPath].
#[typeshare]
pub type FindNodeWithPathResponse = NodeEntity;
