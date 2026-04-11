use mogh_resolver::Resolve;
use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::{
  api::write::CicadaWriteRequest,
  entities::checkpoint::{CheckpointEntity, CheckpointId},
};

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/UpdateCheckpoint",
  description = "Update a checkpoint",
  request_body(content = UpdateCheckpoint),
  responses(
    (status = 200, description = "The updated checkpoint", body = UpdateCheckpointResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn update_checkpoint() {}

/// Update a checkpoint. Response: [UpdateCheckpointResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(UpdateCheckpointResponse)]
#[error(mogh_error::Error)]
pub struct UpdateCheckpoint {
  /// The checkpoint ID
  pub id: CheckpointId,
  /// The name of the checkpoint
  pub name: Option<String>,
  /// The description for the checkpoint
  pub description: Option<String>,
}

/// Response for [UpdateCheckpoint].
#[typeshare]
pub type UpdateCheckpointResponse = CheckpointEntity;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/DeleteCheckpoint",
  description = "Delete a checkpoint",
  request_body(content = DeleteCheckpoint),
  responses(
    (status = 200, description = "The deleted checkpoint", body = DeleteCheckpointResponse),
    (status = 404, description = "Checkpoint not found", body = mogh_error::Serror),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn delete_checkpoint() {}

/// Delete a checkpoint. Response: [DeleteCheckpointResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(DeleteCheckpointResponse)]
#[error(mogh_error::Error)]
pub struct DeleteCheckpoint {
  /// The checkpoint ID
  pub id: CheckpointId,
}

/// Response for [DeleteCheckpoint].
#[typeshare]
pub type DeleteCheckpointResponse = CheckpointEntity;
