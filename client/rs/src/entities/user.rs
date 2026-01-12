use serde::{Deserialize, Serialize};
use serde_json::json;
use surrealdb_types::{RecordId, RecordIdKey, SurrealValue};
use typeshare::typeshare;

use crate::entities::{Iso8601Timestamp, JsonValue};

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UserRecord {
  /// The unique user id
  pub id: UserId,
  /// The name of the user, ie username
  pub name: String,
  /// Whether user is enabled.
  /// Disabled users cannot log in and have no API access.
  #[serde(default)]
  pub enabled: bool,
  // =========
  // = LOGIN =
  // =========
  /// Hashed user password.
  /// Empty if local login is not set.
  #[serde(default)]
  pub password: String,
  /// OIDC subject identifier
  /// Empty if OIDC login is not linked.
  #[serde(default)]
  pub oidc_subject: String,
  /// Github identifier.
  /// Empty if Github login is not linked.
  #[serde(default)]
  pub github_id: String,
  /// Google identifier.
  /// Empty if Google login is not linked.
  #[serde(default)]
  pub google_id: String,
  // =======
  // = 2FA =
  // =======
  /// User passkey config for 2fa
  pub passkey: Option<JsonValue>,
  /// User totp secret.
  #[serde(default)]
  pub totp_secret: String,
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

impl UserRecord {
  pub fn sanitize(&mut self) {
    if !self.password.is_empty() {
      self.password = String::from("redacted");
    }
    if !self.totp_secret.is_empty() {
      self.totp_secret = String::from("redacted");
    }
    if let Some(passkey) = self.passkey.as_mut() {
      *passkey = json!("redacted")
    }
  }
}

#[typeshare(serialized_as = "string")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UserId(pub String);

crate::surreal_id!(UserId, "User");
