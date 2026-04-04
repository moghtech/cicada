use mogh_resolver::{HasResponse, Resolve};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

pub mod device;
pub mod encryption_key;
pub mod filesystem;
pub mod node;
pub mod onboarding_key;
pub mod secret;

//

pub trait CicadaReadRequest: HasResponse {}

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/read/GetVersion",
  description = "Get the version of the Cicada Core API.",
  request_body(content = GetVersion),
  responses(
    (status = 200, description = "Cicada Core API version", body = GetVersionResponse),
  ),
)]
pub fn get_version() {}

/// Get the version of the Cicada Core API.
/// Response: [GetVersionResponse].
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaReadRequest)]
#[response(GetVersionResponse)]
#[error(mogh_error::Error)]
pub struct GetVersion {}

/// Response for [GetVersion].
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetVersionResponse {
  /// The version of the core api.
  pub version: String,
}

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/read/GetUsername",
  description = "Get a user's username by ID.",
  request_body(content = GetUsername),
  responses(
    (status = 200, description = "The user's username", body = GetUsernameResponse),
    (status = 403, description = "User not found", body = mogh_error::Serror),
  ),
)]
pub fn get_username() {}

/// Gets the username of a specific user.
/// Response: [GetUsernameResponse].
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaReadRequest)]
#[response(GetUsernameResponse)]
#[error(mogh_error::Error)]
pub struct GetUsername {
  /// The id of the user
  pub user_id: String,
}

/// Response for [GetUsername].
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetUsernameResponse {
  /// The username of the user.
  pub username: String,
  /// An optional icon for the user.
  pub avatar: Option<String>,
}
