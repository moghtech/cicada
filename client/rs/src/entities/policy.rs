use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::entities::{
  Iso8601Timestamp, filesystem::FilesystemId, node::NodeId,
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
  /// The groups to which this policy applies.
  pub groups: Vec<String>,
  /// Full filesystems the groups can access.
  pub filesystems: Vec<FilesystemId>,
  /// Specific nodes the groups can access.
  /// If the node is a folder, this gives access to all children.
  pub nodes: Vec<NodeId>,
  /// Give the groups write access to configured filesystems and nodes.
  /// Otherwise access is read only.
  pub write: bool,
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
