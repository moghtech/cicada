use mogh_resolver::Resolve;
use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::{
  api::write::CicadaWriteRequest,
  entities::{
    CheckpointingMode, InterpolationMode,
    encryption_key::EncryptionKeyId,
    filesystem::{FilesystemId, FilesystemRecord},
  },
};

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/CreateFilesystem",
  description = "Create a new filesystem",
  request_body(content = CreateFilesystem),
  responses(
    (status = 200, description = "The created filesystem", body = CreateFilesystemResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn create_filesystem() {}

/// Create a filesystem. Response: [CreateFilesystemResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(CreateFilesystemResponse)]
#[error(mogh_error::Error)]
pub struct CreateFilesystem {
  /// The name of the filesystem
  pub name: String,
  /// The default checkpointing mode
  /// - `"Enabled"`
  /// - `"Disabled"`
  pub checkpointing: Option<CheckpointingMode>,
  /// The default interpolation mode
  /// - `"Brackets"` (`[[SECRET]]`)
  /// - `"CurlyBrackets"` (`{{SECRET}}`)
  /// - `"EnvVar"` (`${SECRET}`)
  /// - `"Disabled"`
  pub interpolation: Option<InterpolationMode>,
  /// Choose a specific encryption key.
  /// Otherwise uses the current global default.
  pub encryption_key: Option<EncryptionKeyId>,
}

/// Response for [CreateFilesystem].
#[typeshare]
pub type CreateFilesystemResponse = FilesystemRecord;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/UpdateFilesystem",
  description = "Update a filesystem",
  request_body(content = UpdateFilesystem),
  responses(
    (status = 200, description = "The updated filesystem", body = UpdateFilesystemResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn update_filesystem() {}

/// Update a filesystem. Response: [UpdateFilesystemResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(UpdateFilesystemResponse)]
#[error(mogh_error::Error)]
pub struct UpdateFilesystem {
  /// The filesystem ID
  pub id: FilesystemId,
  /// The name of the filesystem
  pub name: Option<String>,
  /// The default checkpointing mode
  /// - `"Enabled"`
  /// - `"Disabled"`
  pub checkpointing: Option<CheckpointingMode>,
  /// The default interpolation mode
  /// - `"Brackets"` (`[[SECRET]]`)
  /// - `"CurlyBrackets"` (`{{SECRET}}`)
  /// - `"EnvVar"` (`${SECRET}`)
  /// - `"Disabled"`
  pub interpolation: Option<InterpolationMode>,
  /// The default encryption key
  pub encryption_key: Option<EncryptionKeyId>,
}

/// Response for [UpdateFilesystem].
#[typeshare]
pub type UpdateFilesystemResponse = FilesystemRecord;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/DeleteFilesystem",
  description = "Delete a filesystem",
  request_body(content = DeleteFilesystem),
  responses(
    (status = 200, description = "The deleted filesystem", body = DeleteFilesystemResponse),
    (status = 404, description = "Filesystem not found", body = mogh_error::Serror),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn delete_filesystem() {}

/// Delete a filesystem. Response: [DeleteFilesystemResponse].
///
/// WARNING. This will also delete all nodes on the filesystem.
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(DeleteFilesystemResponse)]
#[error(mogh_error::Error)]
pub struct DeleteFilesystem {
  /// The filesystem ID
  pub id: FilesystemId,
}

/// Response for [DeleteFilesystem].
#[typeshare]
pub type DeleteFilesystemResponse = FilesystemRecord;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/BatchDeleteFilesystems",
  description = "Batch delete many filesystems.",
  request_body(content = BatchDeleteFilesystems),
  responses(
    (status = 200, description = "The deleted filesystems", body = BatchDeleteFilesystemsResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn batch_delete_filesystems() {}

/// Batch delete filesystems. Response: [BatchDeleteFilesystemsResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(BatchDeleteFilesystemsResponse)]
#[error(mogh_error::Error)]
pub struct BatchDeleteFilesystems {
  /// The onboarding_key ID
  pub ids: Vec<FilesystemId>,
}

/// Response for [BatchDeleteFilesystems].
#[typeshare]
pub type BatchDeleteFilesystemsResponse = Vec<FilesystemRecord>;
