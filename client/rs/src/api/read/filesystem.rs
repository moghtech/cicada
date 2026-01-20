use mogh_resolver::Resolve;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
  api::read::CicadaReadRequest,
  entities::filesystem::FilesystemRecord,
};

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/read/ListFilesystems",
  description = "List available filesystems",
  request_body(content = ListFilesystems),
  responses(
    (status = 200, description = "List of filesystems", body = ListFilesystemsResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn list_filesystems() {}

/// List filesystems. Response: [ListFilesystemsResponse].
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaReadRequest)]
#[response(ListFilesystemsResponse)]
#[error(mogh_error::Error)]
pub struct ListFilesystems {}

/// Response for [ListFilesystems].
#[typeshare]
pub type ListFilesystemsResponse = Vec<FilesystemRecord>;
