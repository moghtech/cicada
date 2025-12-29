use derive_empty_traits::EmptyTraits;
use resolver_api::Resolve;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use utoipa::ToSchema;

use crate::{
  api::read::CicadaReadRequest,
  entities::{
    U64,
    node::{NodeListItem, NodeRecord},
  },
};

fn default_parent() -> u64 {
  1
}

//

/// List filesystem nodes. Response: [ListNodesResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, Resolve, EmptyTraits, ToSchema,
)]
#[empty_traits(CicadaReadRequest)]
#[response(ListNodesResponse)]
#[error(serror::Error)]
pub struct ListNodes {
  /// Filesystem id
  pub filesystem: String,
  /// parent inode number.
  /// Default: 1 (the root node).
  #[serde(default = "default_parent")]
  pub parent: U64,
}

/// Response for [ListNodes].
#[typeshare]
pub type ListNodesResponse = Vec<NodeListItem>;

//

/// Get a node. Response: [NodeRecord].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, Resolve, EmptyTraits, ToSchema,
)]
#[empty_traits(CicadaReadRequest)]
#[response(GetNodeResponse)]
#[error(serror::Error)]
pub struct GetNode {
  /// inode number
  pub id: U64,
}

/// Response for [GetNode].
#[typeshare]
pub type GetNodeResponse = NodeRecord;

//

/// Find a node using parent inode number and name. Response: [NodeRecord].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, Resolve, EmptyTraits, ToSchema,
)]
#[empty_traits(CicadaReadRequest)]
#[response(FindNodeResponse)]
#[error(serror::Error)]
pub struct FindNode {
  /// Filesystem id
  pub filesystem: String,
  /// parent inode number.
  /// Default: 1 (the root node).
  #[serde(default = "default_parent")]
  pub parent: U64,
  /// file name
  pub name: String,
}

/// Response for [FindNode].
#[typeshare]
pub type FindNodeResponse = NodeRecord;
