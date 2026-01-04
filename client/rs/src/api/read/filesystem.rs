use derive_empty_traits::EmptyTraits;
use resolver_api::Resolve;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
  api::read::CicadaReadRequest,
  entities::filesystem::FilesystemRecord,
};

//

/// List filesystems. Response: [ListFilesystemsResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, Resolve, EmptyTraits,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaReadRequest)]
#[response(ListFilesystemsResponse)]
#[error(mogh_error::Error)]
pub struct ListFilesystems {}

/// Response for [ListFilesystems].
#[typeshare]
pub type ListFilesystemsResponse = Vec<FilesystemRecord>;
