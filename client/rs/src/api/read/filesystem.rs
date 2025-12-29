use derive_empty_traits::EmptyTraits;
use resolver_api::Resolve;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use utoipa::ToSchema;

use crate::{
  api::read::CicadaReadRequest,
  entities::filesystem::FilesystemRecord,
};

//

/// List filesystems. Response: [ListFilesystemsResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, Resolve, EmptyTraits, ToSchema,
)]
#[empty_traits(CicadaReadRequest)]
#[response(ListFilesystemsResponse)]
#[error(serror::Error)]
pub struct ListFilesystems {}

/// Response for [ListFilesystems].
#[typeshare]
pub type ListFilesystemsResponse = Vec<FilesystemRecord>;
