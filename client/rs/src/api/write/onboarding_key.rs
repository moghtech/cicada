use mogh_resolver::Resolve;
use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::{
  api::write::CicadaWriteRequest,
  entities::onboarding_key::{OnboardingKeyId, OnboardingKeyRecord},
};

//

/// Create an onboarding key. Response: [CreateOnboardingKeyResponse].
#[typeshare]
#[derive(
  Debug,
  Clone,
  Serialize,
  Deserialize,
  SurrealValue,
  Resolve,
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

/// Update an onboarding key. Response: [UpdateOnboardingKeyResponse].
#[typeshare]
#[derive(
  Debug,
  Clone,
  Serialize,
  Deserialize,
  SurrealValue,
  Resolve,
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

/// Delete a onboarding_key. Response: [DeleteOnboardingKeyResponse].
#[typeshare]
#[derive(
  Debug,
  Clone,
  Serialize,
  Deserialize,
  SurrealValue,
  Resolve,
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

/// Batch delete onboarding keys. Response: [BatchDeleteOnboardingKeysResponse].
#[typeshare]
#[derive(
  Debug,
  Clone,
  Serialize,
  Deserialize,
  SurrealValue,
  Resolve,
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
