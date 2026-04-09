use mogh_resolver::Resolve;
use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::{
  api::write::CicadaWriteRequest,
  entities::user::{UserId, UserRecord},
};

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/CreateUser",
  description = "Create a new user",
  request_body(content = CreateUser),
  responses(
    (status = 200, description = "The created user", body = CreateUserResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn create_user() {}

/// Create a local user with username and password. Response: [CreateUserResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(CreateUserResponse)]
#[error(mogh_error::Error)]
pub struct CreateUser {
  /// The username of the user
  pub username: String,
  /// The password of the user
  pub password: String,
  /// Whether user is enabled. Default: true
  #[serde(default = "default_enabled")]
  pub enabled: bool,
  /// The groups to assign to user
  #[serde(default)]
  pub groups: Vec<String>,
  /// User has full API access as an administrator.
  #[surreal(default)]
  pub admin: bool,
  /// User can elevate or demote other users admin and super_admin properties.
  #[surreal(default)]
  pub super_admin: bool,
}

fn default_enabled() -> bool {
  true
}

/// Response for [CreateUser].
#[typeshare]
pub type CreateUserResponse = UserRecord;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/UpdateUser",
  description = "Update a user",
  request_body(content = UpdateUser),
  responses(
    (status = 200, description = "The updated user", body = UpdateUserResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn update_user() {}

/// Update a user. Response: [UpdateUserResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(UpdateUserResponse)]
#[error(mogh_error::Error)]
pub struct UpdateUser {
  /// The user ID
  pub id: UserId,
  /// The username of the user
  #[serde(skip_serializing_if = "Option::is_none")]
  pub username: Option<String>,
  /// Whether user is enabled
  #[serde(skip_serializing_if = "Option::is_none")]
  pub enabled: Option<bool>,
}

/// Response for [UpdateUser].
#[typeshare]
pub type UpdateUserResponse = UserRecord;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/DeleteUser",
  description = "Delete a user",
  request_body(content = DeleteUser),
  responses(
    (status = 200, description = "The deleted user", body = DeleteUserResponse),
    (status = 404, description = "User not found", body = mogh_error::Serror),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn delete_user() {}

/// Delete a user. Response: [DeleteUserResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(DeleteUserResponse)]
#[error(mogh_error::Error)]
pub struct DeleteUser {
  /// The user ID
  pub id: UserId,
}

/// Response for [DeleteUser].
#[typeshare]
pub type DeleteUserResponse = UserRecord;
