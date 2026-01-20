use mogh_resolver::Resolve;
use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::{
  api::write::CicadaWriteRequest,
  entities::onboarding_key::{OnboardingKeyId, OnboardingKeyRecord},
};

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/CreateOnboardingKey",
  description = "Create a new onboarding key",
  request_body(content = CreateOnboardingKey),
  responses(
    (status = 200, description = "The created onboarding key", body = CreateOnboardingKeyResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn create_onboarding_key() {}

/// Create an onboarding key. Response: [CreateOnboardingKeyResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(CreateOnboardingKeyResponse)]
#[error(mogh_error::Error)]
pub struct CreateOnboardingKey {
  /// The name of the onboarding_key
  pub name: String,
  /// Optionally provide a pre-existing Spki encoded public key
  /// generated using another method. Otherwise a new keypair will be
  /// generated and the private key returned. The private key will not be
  /// stored otherwise.
  pub public_key: Option<String>,
  /// Whether device is enabled. Default: true
  #[serde(default = "default_enabled")]
  pub enabled: bool,
}

fn default_enabled() -> bool {
  true
}

/// Response for [CreateOnboardingKey].
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateOnboardingKeyResponse {
  /// Pkcs8 encoded private key.
  /// Only present if user *does not* pass pre existing public key to [CreateOnboardingKey].
  pub private_key: Option<String>,
  /// The created onboarding key record
  pub created: OnboardingKeyRecord,
}

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/UpdateOnboardingKey",
  description = "Update an onboarding key",
  request_body(content = UpdateOnboardingKey),
  responses(
    (status = 200, description = "The updated onboarding key", body = UpdateOnboardingKeyResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn update_onboarding_key() {}

/// Update an onboarding key. Response: [UpdateOnboardingKeyResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(UpdateOnboardingKeyResponse)]
#[error(mogh_error::Error)]
pub struct UpdateOnboardingKey {
  /// The onboarding_key ID
  pub id: OnboardingKeyId,
  /// The name of the onboarding key
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  /// The onboarding key public key
  #[serde(skip_serializing_if = "Option::is_none")]
  pub public_key: Option<String>,
  /// Whether the onboarding key is enabled / can onboard.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub enabled: Option<bool>,
}

/// Response for [UpdateOnboardingKey].
#[typeshare]
pub type UpdateOnboardingKeyResponse = OnboardingKeyRecord;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/DeleteOnboardingKey",
  description = "Delete an onboarding key",
  request_body(content = DeleteOnboardingKey),
  responses(
    (status = 200, description = "The deleted onboarding key", body = DeleteOnboardingKeyResponse),
    (status = 404, description = "OnboardingKey not found", body = mogh_error::Serror),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn delete_onboarding_key() {}

/// Delete a onboarding_key. Response: [DeleteOnboardingKeyResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(DeleteOnboardingKeyResponse)]
#[error(mogh_error::Error)]
pub struct DeleteOnboardingKey {
  /// The onboarding_key ID
  pub id: OnboardingKeyId,
}

/// Response for [DeleteOnboardingKey].
#[typeshare]
pub type DeleteOnboardingKeyResponse = OnboardingKeyRecord;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/BatchDeleteOnboardingKeys",
  description = "Batch delete onboarding keys",
  request_body(content = BatchDeleteOnboardingKeys),
  responses(
    (status = 200, description = "The deleted onboarding keys", body = BatchDeleteOnboardingKeysResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn batch_delete_onboarding_keys() {}

/// Batch delete onboarding keys. Response: [BatchDeleteOnboardingKeysResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(BatchDeleteOnboardingKeysResponse)]
#[error(mogh_error::Error)]
pub struct BatchDeleteOnboardingKeys {
  /// The onboarding key IDs
  pub ids: Vec<OnboardingKeyId>,
}

/// Response for [BatchDeleteOnboardingKeys].
#[typeshare]
pub type BatchDeleteOnboardingKeysResponse = Vec<OnboardingKeyRecord>;
