use mogh_resolver::Resolve;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
  api::read::CicadaReadRequest, entities::user::UserEntity,
};

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/read/ListUsers",
  description = "List available users",
  request_body(content = ListUsers),
  responses(
    (status = 200, description = "List of users", body = ListUsersResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn list_users() {}

/// List users. Response: [ListUsersResponse].
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaReadRequest)]
#[response(ListUsersResponse)]
#[error(mogh_error::Error)]
pub struct ListUsers {}

/// Response for [ListUsers].
#[typeshare]
pub type ListUsersResponse = Vec<UserEntity>;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/read/GetUser",
  description = "Get a specific user by id or name",
  request_body(content = GetUser),
  responses(
    (status = 200, description = "The requested user", body = UserEntity),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn get_user() {}

/// Get a specific user by id or name. Response: [GetUserResponse].
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaReadRequest)]
#[response(GetUserResponse)]
#[error(mogh_error::Error)]
pub struct GetUser {
  /// User id or name
  pub id: String,
}

/// Response for [GetUser].
#[typeshare]
pub type GetUserResponse = UserEntity;
