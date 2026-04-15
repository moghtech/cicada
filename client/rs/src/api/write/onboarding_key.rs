use mogh_resolver::Resolve;
use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::{
  api::write::CicadaWriteRequest,
  entities::{
    U64,
    onboarding_key::{OnboardingKeyId, OnboardingKeyRecord},
  },
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
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(CreateOnboardingKeyResponse)]
#[error(mogh_error::Error)]
pub struct CreateOnboardingKey {
  /// The name of the onboarding_key
  pub name: String,
  /// Optionally specify an existing private key, otherwise
  /// generate fresh key. This key is not stored directly,
  /// only the public key.
  pub private_key: Option<String>,
  /// Whether onboarding key is enabled. Default: true
  #[serde(default = "default_enabled")]
  pub enabled: bool,
  /// Expiry timestamp in unix milliseconds.
  /// Passing 0 or ommiting means no expiry.
  #[serde(default)]
  pub expires: U64,
  /// Devices which onboard using this key will automatically
  /// get these groups.
  pub groups: Option<Vec<String>>,
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
  /// If user passes pre existing private key, it will still return it.
  pub private_key: String,
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
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(UpdateOnboardingKeyResponse)]
#[error(mogh_error::Error)]
pub struct UpdateOnboardingKey {
  /// The onboarding_key ID
  pub id: OnboardingKeyId,
  /// The name of the onboarding key
  pub name: Option<String>,
  /// The onboarding key public key
  pub public_key: Option<String>,
  /// Whether the onboarding key is enabled / can onboard.
  pub enabled: Option<bool>,
  /// Devices which onboard using this key will automatically
  /// get these groups.
  pub groups: Option<Vec<String>>,
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
#[surreal(crate = "surrealdb_types")]
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
#[surreal(crate = "surrealdb_types")]
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
