use derive_empty_traits::EmptyTraits;
use resolver_api::{HasResponse, Resolve};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::user::UserRecord;

pub mod filesystem;
pub mod node;

//

pub trait CicadaReadRequest: HasResponse {}

//

/// Get the version of the Cicada Core api.
/// Response: [GetVersionResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, Resolve, EmptyTraits,
)]
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

/// Get calling user extracted from the request authorization.
/// Response: [User].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, Resolve, EmptyTraits,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaReadRequest)]
#[response(GetUserResponse)]
#[error(mogh_error::Error)]
pub struct GetUser {}

#[typeshare]
pub type GetUserResponse = UserRecord;
