use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::entities::{
  device::DeviceId, onboarding_key::OnboardingKeyId,
  policy::PolicyId, user::UserId,
};

/// Group entity produced by database query
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GroupEntity {
  /// The name of the group
  pub name: String,
  /// The users assigned to the group
  pub users: Vec<UserId>,
  /// The devices assigned to the group
  pub devices: Vec<DeviceId>,
  /// The policies granted to the group
  pub policies: Vec<PolicyId>,
  /// The onboarding keys with this group attached
  pub onboarding_keys: Vec<OnboardingKeyId>,
}
