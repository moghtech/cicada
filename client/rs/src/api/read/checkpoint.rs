use mogh_resolver::Resolve;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
  api::read::CicadaReadRequest,
  entities::checkpoint::{
    CheckpointEntity, CheckpointId, CheckpointListItem,
    CheckpointTarget,
  },
};

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/read/ListCheckpoints",
  description = "List available checkpoints for a file",
  request_body(content = ListCheckpoints),
  responses(
    (status = 200, description = "List of checkpoints", body = ListCheckpointsResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn list_checkpoints() {}

/// List checkpoints. Response: [ListCheckpointsResponse].
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaReadRequest)]
#[response(ListCheckpointsResponse)]
#[error(mogh_error::Error)]
pub struct ListCheckpoints {
  /// Get checkpoints for this node (file) or secret
  pub target: CheckpointTarget,
}

/// Response for [ListCheckpoints].
#[typeshare]
pub type ListCheckpointsResponse = Vec<CheckpointListItem>;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/read/GetCheckpoint",
  description = "Get a specific checkpoint by id",
  request_body(content = GetCheckpoint),
  responses(
    (status = 200, description = "The requested checkpoint", body = GetCheckpointResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn get_checkpoint() {}

/// Get a specific checkpoint by id. Response: [GetCheckpointResponse].
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaReadRequest)]
#[response(GetCheckpointResponse)]
#[error(mogh_error::Error)]
pub struct GetCheckpoint {
  /// Checkpoint id
  pub id: CheckpointId,
}

/// Response for [GetCheckpoint].
#[typeshare]
pub type GetCheckpointResponse = CheckpointEntity;
