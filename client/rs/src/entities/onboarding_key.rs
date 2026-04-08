use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::entities::Iso8601Timestamp;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct OnboardingKeyRecord {
  /// The unique onboarding key id
  pub id: OnboardingKeyId,
  /// The name of the onboarding key
  pub name: String,
  /// The onboarding public key.
  /// This is used to authenticate onboarding requests.
  pub public_key: String,
  /// Whether onboarding key is enabled.
  /// Disabled onboarding keys cannot onboard devices.
  pub enabled: bool,
  /// Expiry timestamp, or null for no expiry.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub expires: Option<Iso8601Timestamp>,
  /// Created at as ISO8601 timestamp.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub created_at: Iso8601Timestamp,
  /// Updated at as ISO8601 timestamp.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub updated_at: Iso8601Timestamp,
}

impl OnboardingKeyRecord {
  pub fn sanitize(&mut self) {}
}

#[typeshare(serialized_as = "string")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct OnboardingKeyId(pub String);

crate::surreal_id!(OnboardingKeyId, "OnboardingKey");
