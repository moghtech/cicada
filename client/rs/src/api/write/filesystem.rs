use derive_empty_traits::EmptyTraits;
use resolver_api::Resolve;
use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::{
  api::write::CicadaWriteRequest,
  entities::filesystem::{FilesystemId, FilesystemRecord},
};

//

/// Create filesystem node. Response: [CreateFilesystemResponse].
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
#[response(CreateFilesystemResponse)]
#[error(serror::Error)]
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
#[response(UpdateFilesystemResponse)]
#[error(serror::Error)]
pub struct UpdateFilesystem {
  /// The filesystem ID
  pub id: FilesystemId,
  /// The name of the filesystem
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
#[response(DeleteFilesystemResponse)]
#[error(serror::Error)]
pub struct DeleteFilesystem {
  /// The filesystem ID
  pub id: FilesystemId,
}

/// Response for [DeleteFilesystem].
#[typeshare]
pub type DeleteFilesystemResponse = FilesystemRecord;
