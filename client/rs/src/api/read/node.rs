use derive_empty_traits::EmptyTraits;
use resolver_api::Resolve;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
  api::read::CicadaReadRequest,
  entities::{U64, node::NodeRecord},
};

//

/// List filesystem nodes. Response: [ListNodesResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, Resolve, EmptyTraits,
)]
#[empty_traits(CicadaReadRequest)]
#[response(ListNodesResponse)]
#[error(serror::Error)]
pub struct ListNodes {}

/// Response for [ListNodes].
#[typeshare]
pub type ListNodesResponse = Vec<NodeRecord>;

//

/// Get a node. Response: [NodeRecord].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, Resolve, EmptyTraits,
)]
#[empty_traits(CicadaReadRequest)]
#[response(GetNodeResponse)]
#[error(serror::Error)]
pub struct GetNode {
  pub ino: U64,
}

/// Response for [GetNode].
#[typeshare]
pub type GetNodeResponse = NodeRecord;
