use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::entities::{Iso8601Timestamp, user::UserId};

/// Call the Cicada API using API keys.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ApiKeyRecord {
  /// The unique user login id
  pub id: ApiKeyId,
  /// The user which this api key is for
  pub user: UserId,
  /// Name of the api key
  pub name: String,
  /// Unique key associated with secret
  pub key: String,
  /// Hash of the secret
  pub secret: Option<String>,
  /// Whether api key is enabled.
  /// Disabled api keys cannot access the api.
  pub enabled: bool,
  /// Expiry timestamp, or null for no expiry.
  #[cfg_attr(feature = "utoipa", schema(value_type = Option<String>))]
  pub expires: Option<Iso8601Timestamp>,
  // ===============
  // = TIMESTAMPS =
  // ===============
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
pub struct ApiKeyId(pub String);

crate::surreal_id!(ApiKeyId, "ApiKey");
