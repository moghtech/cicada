use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use surrealdb_types::{RecordId, RecordIdKey, SurrealValue};
use typeshare::typeshare;

use crate::entities::Iso8601Timestamp;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UserRecord {
  /// The unique user id
  pub id: UserId,
  /// The name of the user, ie username
  pub name: String,
  /// Hashed user password.
  /// Empty if local login is not set.
  pub password: String,
  /// Whether user is enabled.
  /// Disabled users cannot log in and have no API access.
  #[serde(default)]
  pub enabled: bool,
  /// Created at as ISO8601 timestamp.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub created_at: Iso8601Timestamp,
  /// Updated at as ISO8601 timestamp.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub updated_at: Iso8601Timestamp,
}

impl UserRecord {
  pub fn sanitize(&mut self) {
    self.password.clear();
  }
}

#[typeshare(serialized_as = "string")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UserId(pub String);

crate::surreal_id!(UserId, "User");
