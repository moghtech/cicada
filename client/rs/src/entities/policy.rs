use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::entities::{
  Iso8601Timestamp, device::DeviceId, filesystem::FilesystemId,
  user::UserId,
};

/// Give groups access to filesystems and nodes
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PolicyRecord {
  /// The unique policy id
  pub id: PolicyId,
  /// The name of the policy. Must be unique.
  pub name: String,
  /// Whether the policy is enabled.
  pub enabled: bool,
  /// The users to which this policy applies
  pub users: Vec<UserId>,
  /// The devices to which this policy applies
  pub devices: Vec<DeviceId>,
  /// The groups to which this policy applies.
  pub groups: Vec<String>,
  /// Filesystems the users / devices / groups can access.
  pub filesystems: Vec<FilesystemId>,
  /// Give the groups write access to configured filesystems.
  /// Otherwise access is read only.
  pub filesystem_write: bool,
  /// Created at as ISO8601 timestamp.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub created_at: Iso8601Timestamp,
  /// Updated at as ISO8601 timestamp.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub updated_at: Iso8601Timestamp,
}

#[typeshare(serialized_as = "string")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PolicyId(pub String);

crate::surreal_id!(PolicyId, "Policy");
