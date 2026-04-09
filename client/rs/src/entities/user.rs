use serde::{Deserialize, Serialize};
use serde_json::json;
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::entities::{
  Iso8601Timestamp, JsonValue, external_login::ExternalLoginRecord,
};

/// Users queryable from the API
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UserEntity {
  /// The unique user id
  pub id: UserId,
  /// The name of the user, ie username
  pub username: String,
  /// Link for user avatar, or empty string.
  pub avatar: String,
  /// Whether user is enabled.
  /// Disabled users cannot log in and have no API access.
  pub enabled: bool,
  /// Whether user has password set.
  pub password: bool,
  /// The external login methods the user has set.
  pub external_logins: Vec<ExternalLoginRecord>,
  // =======
  // = 2FA =
  // =======
  /// Whether user is enrolled in passkey 2fa
  pub passkey: bool,
  /// Whether user is enrolled in TOTP 2fa
  pub totp: bool,
  /// Allow external logins to skip 2fa.
  pub external_skip_2fa: bool,
  // ===============
  // = PERMISSIONS =
  // ===============
  /// The groups to which this user belongs.
  #[surreal(default)]
  pub groups: Vec<String>,
  /// User has full API access as an administrator.
  #[surreal(default)]
  pub admin: bool,
  /// User can elevate or demote other users admin and super_admin properties.
  #[surreal(default)]
  pub super_admin: bool,
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

/// Users on the database
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UserRecord {
  /// The unique user id
  pub id: UserId,
  /// The name of the user, ie username
  pub username: String,
  /// Link for user avatar, or empty string.
  pub avatar: String,
  /// Whether user is enabled.
  /// Disabled users cannot log in and have no API access.
  pub enabled: bool,
  /// Hashed user password.
  /// Empty if local login is not set.
  pub password: String,
  // =======
  // = 2FA =
  // =======
  /// User passkey config for 2fa
  pub passkey: Option<JsonValue>,
  /// User totp secret.
  /// TODO: encryption
  pub totp_secret: String,
  /// Allow external logins to skip 2fa.
  pub external_skip_2fa: bool,
  // ===============
  // = PERMISSIONS =
  // ===============
  /// The groups to which this user belongs.
  #[surreal(default)]
  pub groups: Vec<String>,
  /// User has full API access as an administrator.
  #[surreal(default)]
  pub admin: bool,
  /// User can elevate other users to admin
  #[surreal(default)]
  pub super_admin: bool,
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
