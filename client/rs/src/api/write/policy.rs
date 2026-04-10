use mogh_resolver::Resolve;
use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::{
  api::write::CicadaWriteRequest,
  entities::{
    device::DeviceId,
    filesystem::FilesystemId,
    policy::{PolicyId, PolicyRecord},
    user::UserId,
  },
};

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/CreatePolicy",
  description = "Create a new policy",
  request_body(content = CreatePolicy),
  responses(
    (status = 200, description = "The created policy", body = CreatePolicyResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn create_policy() {}

/// Create a policy. Response: [CreatePolicyResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(CreatePolicyResponse)]
#[error(mogh_error::Error)]
pub struct CreatePolicy {
  /// The name of the policy
  pub name: String,
  /// The users to which this policy applies
  pub users: Option<Vec<UserId>>,
  /// The devices to which this policy applies
  pub devices: Option<Vec<DeviceId>>,
  /// The groups to which this policy applies.
  pub groups: Option<Vec<String>>,
  /// Filesystems the users / devices / groups can access.
  pub filesystems: Option<Vec<FilesystemId>>,
  /// Give the groups write access to configured filesystems.
  /// Otherwise access is read only.
  #[serde(default)]
  pub filesystem_write: bool,
}

/// Response for [CreatePolicy].
#[typeshare]
pub type CreatePolicyResponse = PolicyRecord;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/UpdatePolicy",
  description = "Update a policy",
  request_body(content = UpdatePolicy),
  responses(
    (status = 200, description = "The updated policy", body = UpdatePolicyResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn update_policy() {}

/// Update a policy. Response: [UpdatePolicyResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(UpdatePolicyResponse)]
#[error(mogh_error::Error)]
pub struct UpdatePolicy {
  /// The policy ID
  pub id: PolicyId,
  /// The name of the policy
  pub name: Option<String>,
  /// The users to which this policy applies
  pub users: Option<Vec<UserId>>,
  /// The devices to which this policy applies
  pub devices: Option<Vec<DeviceId>>,
  /// The groups to which this policy applies.
  pub groups: Option<Vec<String>>,
  /// Filesystems the users / devices / groups can access.
  pub filesystems: Option<Vec<FilesystemId>>,
  /// Give the groups write access to configured filesystems.
  /// Otherwise access is read only.
  pub filesystem_write: Option<bool>,
}

/// Response for [UpdatePolicy].
#[typeshare]
pub type UpdatePolicyResponse = PolicyRecord;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/DeletePolicy",
  description = "Delete a policy",
  request_body(content = DeletePolicy),
  responses(
    (status = 200, description = "The deleted policy", body = DeletePolicyResponse),
    (status = 404, description = "Policy not found", body = mogh_error::Serror),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn delete_policy() {}

/// Delete a policy. Response: [DeletePolicyResponse].
///
/// WARNING. This will also delete all nodes on the policy.
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(DeletePolicyResponse)]
#[error(mogh_error::Error)]
pub struct DeletePolicy {
  /// The policy ID
  pub id: PolicyId,
}

/// Response for [DeletePolicy].
#[typeshare]
pub type DeletePolicyResponse = PolicyRecord;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/BatchDeletePolicies",
  description = "Batch delete policies",
  request_body(content = BatchDeletePolicies),
  responses(
    (status = 200, description = "The deleted policies", body = BatchDeletePoliciesResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn batch_delete_policies() {}

/// Batch delete policies. Response: [BatchDeletePoliciesResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(BatchDeletePoliciesResponse)]
#[error(mogh_error::Error)]
pub struct BatchDeletePolicies {
  /// The policy IDs
  pub ids: Vec<PolicyId>,
}

/// Response for [BatchDeletePolicies].
#[typeshare]
pub type BatchDeletePoliciesResponse = Vec<PolicyRecord>;
