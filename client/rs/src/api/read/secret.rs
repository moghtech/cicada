use mogh_resolver::Resolve;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
  api::read::CicadaReadRequest,
  entities::{
    U64,
    filesystem::FilesystemId,
    secret::{SecretEntity, SecretId, SecretListItem},
  },
};

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/read/ListSecrets",
  description = "List available secrets.",
  request_body(content = ListSecrets),
  responses(
    (status = 200, description = "List of secrets", body = ListSecretsResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn list_secrets() {}

/// List secrets. Response: [ListSecretsResponse].
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaReadRequest)]
#[response(ListSecretsResponse)]
#[error(mogh_error::Error)]
pub struct ListSecrets {
  /// Filesystem id
  pub filesystem: Option<FilesystemId>,
  /// parent isecret number.
  #[cfg_attr(feature = "utoipa", schema(minimum = 1))]
  pub parent: Option<U64>,
}

/// Response for [ListSecrets].
#[typeshare]
pub type ListSecretsResponse = Vec<SecretListItem>;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/read/GetSecret",
  description = "Get a secret by id",
  request_body(content = GetSecret),
  responses(
    (status = 200, description = "The secret", body = GetSecretResponse),
    (status = 404, description = "Failed to find secret with given id", body = mogh_error::Serror),
    (status = 500, description = "Request failed", body = mogh_error::Serror),
  ),
)]
pub fn get_secret() {}

/// Get a secret. Response: [SecretEntity].
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaReadRequest)]
#[response(GetSecretResponse)]
#[error(mogh_error::Error)]
pub struct GetSecret {
  /// The secret id
  pub id: SecretId,
}

/// Response for [GetSecret].
#[typeshare]
pub type GetSecretResponse = SecretEntity;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/read/FindSecret",
  description = "Find a secret by name",
  request_body(content = FindSecret),
  responses(
    (status = 200, description = "The secret", body = FindSecretResponse),
    (status = 404, description = "Failed to find secret with given parameters", body = mogh_error::Serror),
    (status = 500, description = "Request failed", body = mogh_error::Serror),
  ),
)]
pub fn find_secret() {}

/// Find a secret by name. Response: [SecretEntity].
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaReadRequest)]
#[response(FindSecretResponse)]
#[error(mogh_error::Error)]
pub struct FindSecret {
  /// secret name
  pub name: String,
}

impl FindSecret {
  pub fn with_name(name: impl Into<String>) -> FindSecret {
    FindSecret { name: name.into() }
  }
}

/// Response for [FindSecret].
#[typeshare]
pub type FindSecretResponse = SecretEntity;
