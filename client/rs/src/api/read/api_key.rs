use mogh_resolver::Resolve;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
  api::read::CicadaReadRequest, entities::api_key::ApiKeyRecord,
};

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/read/ListApiKeys",
  description = "List api keys for the calling user.",
  request_body(content = ListApiKeys),
  responses(
    (status = 200, description = "List of api keys", body = ListApiKeysResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn list_api_keys() {}

/// List api keys. Response: [ListApiKeysResponse].
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaReadRequest)]
#[response(ListApiKeysResponse)]
#[error(mogh_error::Error)]
pub struct ListApiKeys {}

/// Response for [ListApiKeys].
#[typeshare]
pub type ListApiKeysResponse = Vec<ApiKeyRecord>;
