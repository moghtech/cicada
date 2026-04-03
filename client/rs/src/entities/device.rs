use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::entities::Iso8601Timestamp;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct DeviceRecord {
  /// The unique Device id
  pub id: DeviceId,
  /// The name of the Device
  pub name: String,
  /// The device public key.
  /// This is used to authenticate device requests.
  pub public_key: String,
  /// Whether device is enabled.
  /// Disabled devices cannot access any files.
  pub enabled: bool,
  /// Created at as ISO8601 timestamp.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub created_at: Iso8601Timestamp,
  /// Updated at as ISO8601 timestamp.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub updated_at: Iso8601Timestamp,
}

impl DeviceRecord {
  pub fn sanitize(&mut self) {}
}

#[typeshare(serialized_as = "string")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct DeviceId(pub String);

crate::surreal_id!(DeviceId, "Device");
