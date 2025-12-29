use derive_empty_traits::EmptyTraits;
use resolver_api::{HasResponse, Resolve};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use utoipa::ToSchema;

pub mod filesystem;
pub mod node;

//

pub trait CicadaReadRequest: HasResponse {}

//

/// Get the version of the Cicada Core api.
/// Response: [GetVersionResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, Resolve, EmptyTraits, ToSchema,
)]
#[empty_traits(CicadaReadRequest)]
#[response(GetVersionResponse)]
#[error(serror::Error)]
pub struct GetVersion {}

/// Response for [GetVersion].
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct GetVersionResponse {
  /// The version of the core api.
  pub version: String,
}
