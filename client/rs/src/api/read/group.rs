use mogh_resolver::Resolve;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
  api::read::CicadaReadRequest, entities::group::GroupEntity,
};

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/read/ListGroups",
  description = "List available groups",
  request_body(content = ListGroups),
  responses(
    (status = 200, description = "List of groups", body = ListGroupsResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn list_groups() {}

/// List groups. Response: [ListGroupsResponse].
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaReadRequest)]
#[response(ListGroupsResponse)]
#[error(mogh_error::Error)]
pub struct ListGroups {}

/// Response for [ListGroups].
#[typeshare]
pub type ListGroupsResponse = Vec<GroupEntity>;
