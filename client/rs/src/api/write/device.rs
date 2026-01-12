use derive_empty_traits::EmptyTraits;
use resolver_api::Resolve;
use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::{
  api::write::CicadaWriteRequest,
  entities::device::{DeviceId, DeviceRecord},
};

//

/// Create a device. Response: [CreateDeviceResponse].
#[typeshare]
#[derive(
  Debug,
  Clone,
  Serialize,
  Deserialize,
  SurrealValue,
  Resolve,
  EmptyTraits,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(CreateDeviceResponse)]
#[error(mogh_error::Error)]
pub struct CreateDevice {
  /// The name of the device
  pub name: String,
  /// The public key of the device
  pub public_key: String,
  /// Whether device is enabled. Default: true
  #[serde(default = "default_enabled")]
  pub enabled: bool,
}

fn default_enabled() -> bool {
  true
}

/// Response for [CreateDevice].
#[typeshare]
pub type CreateDeviceResponse = DeviceRecord;

//

/// Update a device. Response: [UpdateDeviceResponse].
#[typeshare]
#[derive(
  Debug,
  Clone,
  Serialize,
  Deserialize,
  SurrealValue,
  Resolve,
  EmptyTraits,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(UpdateDeviceResponse)]
#[error(mogh_error::Error)]
pub struct UpdateDevice {
  /// The device ID
  pub id: DeviceId,
  /// The name of the device
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  /// The public key of the device
  #[serde(skip_serializing_if = "Option::is_none")]
  pub public_key: Option<String>,
  /// Whether the device is enabled / has access.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub enabled: Option<bool>,
}

/// Response for [UpdateDevice].
#[typeshare]
pub type UpdateDeviceResponse = DeviceRecord;

//

/// Delete a device. Response: [DeleteDeviceResponse].
///
/// WARNING. This will also delete all nodes on the device.
#[typeshare]
#[derive(
  Debug,
  Clone,
  Serialize,
  Deserialize,
  SurrealValue,
  Resolve,
  EmptyTraits,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(DeleteDeviceResponse)]
#[error(mogh_error::Error)]
pub struct DeleteDevice {
  /// The device ID
  pub id: DeviceId,
}

/// Response for [DeleteDevice].
#[typeshare]
pub type DeleteDeviceResponse = DeviceRecord;
