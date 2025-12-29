use derive_empty_traits::EmptyTraits;
use resolver_api::Resolve;
use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;
use utoipa::ToSchema;

use crate::{
  api::write::CicadaWriteRequest,
  entities::filesystem::FilesystemRecord,
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
  ToSchema,
)]
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

/// Update a filesystem node. Response: [UpdateFilesystemResponse].
#[typeshare]
#[derive(
  Debug,
  Clone,
  Serialize,
  Deserialize,
  SurrealValue,
  Resolve,
  EmptyTraits,
  ToSchema,
)]
#[empty_traits(CicadaWriteRequest)]
#[response(UpdateFilesystemResponse)]
#[error(serror::Error)]
pub struct UpdateFilesystem {
  /// The filesystem ID
  pub id: String,
  /// The name of the filesystem
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
}

/// Response for [UpdateFilesystem].
#[typeshare]
pub type UpdateFilesystemResponse = FilesystemRecord;
