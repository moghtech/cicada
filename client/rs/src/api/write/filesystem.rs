use mogh_resolver::Resolve;
use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::{
  api::write::CicadaWriteRequest,
  entities::filesystem::{FilesystemId, FilesystemRecord},
};

//

/// Create a filesystem. Response: [CreateFilesystemResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(CreateFilesystemResponse)]
#[error(mogh_error::Error)]
pub struct CreateFilesystem {
  /// The name of the filesystem
  pub name: String,
}

/// Response for [CreateFilesystem].
#[typeshare]
pub type CreateFilesystemResponse = FilesystemRecord;

//

/// Update a filesystem. Response: [UpdateFilesystemResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(UpdateFilesystemResponse)]
#[error(mogh_error::Error)]
pub struct UpdateFilesystem {
  /// The filesystem ID
  pub id: FilesystemId,
  /// The name of the filesystem
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
}

/// Response for [UpdateFilesystem].
#[typeshare]
pub type UpdateFilesystemResponse = FilesystemRecord;

//

/// Delete a filesystem. Response: [DeleteFilesystemResponse].
///
/// WARNING. This will also delete all nodes on the filesystem.
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
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
