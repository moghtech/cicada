use mogh_resolver::Resolve;
use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::{
  api::write::CicadaWriteRequest,
  entities::{
    checkpoint::{CheckpointEntity, CheckpointId},
    encryption_key::EncryptionKeyId,
  },
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
  path = "/write/UpdateCheckpointEncryptionKey",
  description = "Update a checkpoint's data",
  request_body(content = UpdateCheckpoint),
  responses(
    (status = 200, description = "The updated checkpoint", body = UpdateCheckpointEncryptionKeyResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn update_checkpoint_encryption_key() {}

/// Update a checkpoint's encryption key. Response: [UpdateCheckpointEncryptionKeyResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(UpdateCheckpointEncryptionKeyResponse)]
#[error(mogh_error::Error)]
pub struct UpdateCheckpointEncryptionKey {
  /// The checkpoint id
  pub id: CheckpointId,
  /// Update the encryption key used as master in the envelope encryption.
  pub encryption_key: EncryptionKeyId,
  /// Whether to interpolate secrets into returned file contents
  #[serde(default)]
  pub interpolated: bool,
}

/// Response for [UpdateCheckpointEncryptionKey].
#[typeshare]
pub type UpdateCheckpointEncryptionKeyResponse = CheckpointEntity;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/RotateCheckpointEnvelopeKey",
  description = "Update a checkpoint's data",
  request_body(content = UpdateCheckpoint),
  responses(
    (status = 200, description = "The updated checkpoint", body = RotateCheckpointEnvelopeKeyResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn rotate_checkpoint_envelope_key() {}

/// Rotate a checkpoint's envelope encryption key. Response: [RotateCheckpointEnvelopeKeyResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(RotateCheckpointEnvelopeKeyResponse)]
#[error(mogh_error::Error)]
pub struct RotateCheckpointEnvelopeKey {
  /// The checkpoint id
  pub id: CheckpointId,
  /// Whether to interpolate secrets into returned file contents
  #[serde(default)]
  pub interpolated: bool,
}

/// Response for [RotateCheckpointEnvelopeKey].
#[typeshare]
pub type RotateCheckpointEnvelopeKeyResponse = CheckpointEntity;

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

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/BatchDeleteCheckpoints",
  description = "Batch delete many checkpoints.",
  request_body(content = BatchDeleteCheckpoints),
  responses(
    (status = 200, description = "The deleted checkpoints", body = BatchDeleteCheckpointsResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn batch_delete_checkpoints() {}

/// Batch delete checkpoints. Response: [BatchDeleteCheckpointsResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(BatchDeleteCheckpointsResponse)]
#[error(mogh_error::Error)]
pub struct BatchDeleteCheckpoints {
  /// The onboarding_key ID
  pub ids: Vec<CheckpointId>,
}

/// Response for [BatchDeleteCheckpoints].
#[typeshare]
pub type BatchDeleteCheckpointsResponse = Vec<CheckpointEntity>;
