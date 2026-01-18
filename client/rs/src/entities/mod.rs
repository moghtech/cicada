use std::str::FromStr as _;

use mogh_error::anyhow::{self, Context as _};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::entities::encryption_key::EncryptionKeyId;

/// Configuration for Cicada
pub mod config;
/// A device mounting Cicada files
pub mod device;
/// Master encryption keys.
pub mod encryption_key;
/// Login methods for users.
pub mod external_login;
/// Represents virtual filesystems which can be mounted to clients.
pub mod filesystem;
/// Nodes represent entries in a filesystem.
/// They represent either Files or Folders.
pub mod node;
/// Onboard device access.
pub mod onboarding_key;
/// Cicada users.
pub mod user;

#[typeshare(serialized_as = "number")]
pub type U64 = u64;
#[typeshare(serialized_as = "string")]
pub type Iso8601Timestamp = surrealdb_types::Datetime;
#[typeshare(serialized_as = "any")]
pub type JsonValue = serde_json::Value;

/// Represents an empty json object: `{}`
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NoData {}

/// Stored as nested record fields for data the requires application level encryption.
/// Implements envelope encryption, ensuring encryption master key rotation
/// doesn't require re-encrypting the data itself.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct EncryptedData {
  // /// The master encryption key
  pub encryption_key: EncryptionKeyId,
  /// The field key, encrypted with encryption key,
  /// and base64 encoded.
  pub key: String,
  // /// Unencrypted, base64 encoded random nonce used to
  // /// encrypt the 'key' with the encryption key.
  // /// May be empty string when using external KMS.
  pub key_nonce: String,
  /// Encrypted using the (decrypted) field key + data_nonce,
  /// and base64 encoded.
  pub data: String,
  /// Unencrypted, base64 encoded random nonce used to
  /// encrypt the 'data' with this [EncryptedData::key]
  pub data_nonce: String,
}

#[typeshare]
#[derive(
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  Hash,
  Default,
  Serialize,
  Deserialize,
  Display,
  EnumString,
  AsRefStr,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ClientType {
  #[default]
  User,
  Device,
  OnboardingKey,
}

#[typeshare]
#[derive(
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  Hash,
  Default,
  Serialize,
  Deserialize,
  Display,
  EnumString,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Timelength {
  /// `1-sec`
  #[serde(rename = "1-sec")]
  #[strum(serialize = "1-sec")]
  OneSecond,
  /// `1-sec`
  #[serde(rename = "2-sec")]
  #[strum(serialize = "2-sec")]
  TwoSeconds,
  /// `1-sec`
  #[serde(rename = "3-sec")]
  #[strum(serialize = "3-sec")]
  ThreeSeconds,
  /// `5-sec`
  #[serde(rename = "5-sec")]
  #[strum(serialize = "5-sec")]
  FiveSeconds,
  /// `10-sec`
  #[serde(rename = "10-sec")]
  #[strum(serialize = "10-sec")]
  TenSeconds,
  /// `15-sec`
  #[serde(rename = "15-sec")]
  #[strum(serialize = "15-sec")]
  FifteenSeconds,
  /// `30-sec`
  #[serde(rename = "30-sec")]
  #[strum(serialize = "30-sec")]
  ThirtySeconds,
  #[default]
  /// `1-min`
  #[serde(rename = "1-min")]
  #[strum(serialize = "1-min")]
  OneMinute,
  /// `2-min`
  #[serde(rename = "2-min")]
  #[strum(serialize = "2-min")]
  TwoMinutes,
  /// `3-min`
  #[serde(rename = "3-min")]
  #[strum(serialize = "3-min")]
  ThreeMinutes,
  /// `5-min`
  #[serde(rename = "5-min")]
  #[strum(serialize = "5-min")]
  FiveMinutes,
  /// `10-min`
  #[serde(rename = "10-min")]
  #[strum(serialize = "10-min")]
  TenMinutes,
  /// `15-min`
  #[serde(rename = "15-min")]
  #[strum(serialize = "15-min")]
  FifteenMinutes,
  /// `30-min`
  #[serde(rename = "30-min")]
  #[strum(serialize = "30-min")]
  ThirtyMinutes,
  /// `1-hr`
  #[serde(rename = "1-hr")]
  #[strum(serialize = "1-hr")]
  OneHour,
  /// `2-hr`
  #[serde(rename = "2-hr")]
  #[strum(serialize = "2-hr")]
  TwoHours,
  /// `3-hr`
  #[serde(rename = "3-hr")]
  #[strum(serialize = "3-hr")]
  ThreeHours,
  /// `6-hr`
  #[serde(rename = "6-hr")]
  #[strum(serialize = "6-hr")]
  SixHours,
  /// `8-hr`
  #[serde(rename = "8-hr")]
  #[strum(serialize = "8-hr")]
  EightHours,
  /// `12-hr`
  #[serde(rename = "12-hr")]
  #[strum(serialize = "12-hr")]
  TwelveHours,
  /// `1-day`
  #[serde(rename = "1-day")]
  #[strum(serialize = "1-day")]
  OneDay,
  /// `2-day`
  #[serde(rename = "2-day")]
  #[strum(serialize = "2-day")]
  TwoDays,
  /// `3-day`
  #[serde(rename = "3-day")]
  #[strum(serialize = "3-day")]
  ThreeDays,
  /// `1-wk`
  #[serde(rename = "1-wk")]
  #[strum(serialize = "1-wk")]
  OneWeek,
  /// `2-wk`
  #[serde(rename = "2-wk")]
  #[strum(serialize = "2-wk")]
  TwoWeeks,
  /// `30-day`
  #[serde(rename = "30-day")]
  #[strum(serialize = "30-day")]
  ThirtyDays,
}

impl TryInto<async_timing_util::Timelength> for Timelength {
  type Error = anyhow::Error;
  fn try_into(
    self,
  ) -> Result<async_timing_util::Timelength, Self::Error> {
    async_timing_util::Timelength::from_str(&self.to_string())
      .context("Failed to parse timelength?")
  }
}

#[macro_export]
macro_rules! surreal_id {
  ($typ:ident, $table:expr) => {
    impl $typ {
      pub fn as_record_id(&self) -> RecordId {
        surrealdb_types::RecordId::new($table, self.0.as_str())
      }
    }

    impl SurrealValue for $typ {
      fn kind_of() -> surrealdb_types::Kind {
        surrealdb_types::Kind::Record(vec![])
      }

      fn into_value(self) -> surrealdb_types::Value {
        surrealdb_types::Value::RecordId(
          surrealdb_types::RecordId::new($table, self.0),
        )
      }

      fn from_value(
        value: surrealdb_types::Value,
      ) -> mogh_error::anyhow::Result<Self>
      where
        Self: Sized,
      {
        let surrealdb_types::Value::RecordId(id) = value else {
          return Err(mogh_error::anyhow::anyhow!(
            "Value is not RecordId"
          ));
        };
        let RecordIdKey::String(id) = id.key else {
          return Err(mogh_error::anyhow::anyhow!(
            "RecordIdKey is not String"
          ));
        };
        Ok(Self(id))
      }
    }
  };
}
