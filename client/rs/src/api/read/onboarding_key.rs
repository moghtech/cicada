use derive_empty_traits::EmptyTraits;
use resolver_api::Resolve;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
  api::read::CicadaReadRequest,
  entities::onboarding_key::{OnboardingKeyId, OnboardingKeyRecord},
};

//

/// List onboarding keys. Response: [ListOnboardingKeysResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, Resolve, EmptyTraits,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaReadRequest)]
#[response(ListOnboardingKeysResponse)]
#[error(mogh_error::Error)]
pub struct ListOnboardingKeys {}

/// Response for [ListOnboardingKeys].
#[typeshare]
pub type ListOnboardingKeysResponse = Vec<OnboardingKeyRecord>;

//

/// Get an onboarding key. Response: [OnboardingKeyRecord].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, Resolve, EmptyTraits,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaReadRequest)]
#[response(GetOnboardingKeyResponse)]
#[error(mogh_error::Error)]
pub struct GetOnboardingKey {
  /// The onboarding key id
  pub id: OnboardingKeyId,
}

/// Response for [GetOnboardingKey].
#[typeshare]
pub type GetOnboardingKeyResponse = OnboardingKeyRecord;
