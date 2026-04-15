use mogh_resolver::Resolve;
use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::{
  api::write::CicadaWriteRequest,
  entities::api_key::{ApiKeyId, ApiKeyRecord},
};

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/UpdateApiKey",
  description = "Update an api key",
  request_body(content = UpdateApiKey),
  responses(
    (status = 200, description = "The updated api key", body = UpdateApiKeyResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn update_api_key() {}

/// Update an api key. Response: [UpdateApiKeyResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(UpdateApiKeyResponse)]
#[error(mogh_error::Error)]
pub struct UpdateApiKey {
  /// The api_key ID
  pub id: ApiKeyId,
  /// The name of the api key
  pub name: Option<String>,
  /// Whether the api key is enabled / can onboard.
  pub enabled: Option<bool>,
}

/// Response for [UpdateApiKey].
#[typeshare]
pub type UpdateApiKeyResponse = ApiKeyRecord;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/BatchDeleteApiKeys",
  description = "Batch delete api keys",
  request_body(content = BatchDeleteApiKeys),
  responses(
    (status = 200, description = "The deleted api keys", body = BatchDeleteApiKeysResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn batch_delete_api_keys() {}

/// Batch delete api keys. Response: [BatchDeleteApiKeysResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(BatchDeleteApiKeysResponse)]
#[error(mogh_error::Error)]
pub struct BatchDeleteApiKeys {
  /// The api key IDs
  pub ids: Vec<ApiKeyId>,
}

/// Response for [BatchDeleteApiKeys].
#[typeshare]
pub type BatchDeleteApiKeysResponse = Vec<ApiKeyRecord>;
