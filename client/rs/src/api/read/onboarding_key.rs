use mogh_resolver::Resolve;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
  api::read::CicadaReadRequest,
  entities::onboarding_key::{OnboardingKeyId, OnboardingKeyRecord},
};

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/read/ListOnboardingKeys",
  description = "List onboarding keys.",
  request_body(content = ListOnboardingKeys),
  responses(
    (status = 200, description = "List of onboarding keys", body = ListOnboardingKeysResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn list_onboarding_keys() {}

/// List onboarding keys. Response: [ListOnboardingKeysResponse].
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaReadRequest)]
#[response(ListOnboardingKeysResponse)]
#[error(mogh_error::Error)]
pub struct ListOnboardingKeys {}

/// Response for [ListOnboardingKeys].
#[typeshare]
pub type ListOnboardingKeysResponse = Vec<OnboardingKeyRecord>;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/read/GetOnboardingKey",
  description = "Get an onboarding key by id.",
  request_body(content = GetOnboardingKey),
  responses(
    (status = 200, description = "The onboarding key", body = GetOnboardingKeyResponse),
    (status = 404, description = "Failed to find onboarding key with given id", body = mogh_error::Serror),
    (status = 500, description = "Request failed", body = mogh_error::Serror),
  ),
)]
pub fn get_onboarding_key() {}

/// Get an onboarding key by id. Response: [OnboardingKeyRecord].
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
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
