use mogh_resolver::Resolve;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
  api::read::CicadaReadRequest, entities::policy::PolicyRecord,
};

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/read/ListPolicies",
  description = "List available policies",
  request_body(content = ListPolicies),
  responses(
    (status = 200, description = "List of policies", body = ListPoliciesResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn list_policies() {}

/// List policies. Response: [ListPoliciesResponse].
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaReadRequest)]
#[response(ListPoliciesResponse)]
#[error(mogh_error::Error)]
pub struct ListPolicies {}

/// Response for [ListPolicies].
#[typeshare]
pub type ListPoliciesResponse = Vec<PolicyRecord>;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/read/GetPolicy",
  description = "Get a specific policy by id or name",
  request_body(content = GetPolicy),
  responses(
    (status = 200, description = "The requested policy", body = PolicyRecord),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn get_policy() {}

/// Get a specific policy by id or name. Response: [GetPolicyResponse].
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaReadRequest)]
#[response(GetPolicyResponse)]
#[error(mogh_error::Error)]
pub struct GetPolicy {
  /// Policy id or name
  pub id: String,
}

/// Response for [GetPolicy].
#[typeshare]
pub type GetPolicyResponse = PolicyRecord;
