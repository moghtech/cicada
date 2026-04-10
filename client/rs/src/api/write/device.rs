use mogh_resolver::Resolve;
use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::{
  api::write::CicadaWriteRequest,
  entities::device::{DeviceId, DeviceRecord},
};

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/CreateDevice",
  description = "Create a new device",
  request_body(content = CreateDevice),
  responses(
    (status = 200, description = "The created device", body = CreateDeviceResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn create_device() {}

/// Create a device. Response: [CreateDeviceResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
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
  /// The groups this device is a member of
  #[serde(default)]
  pub groups: Vec<String>,
}

fn default_enabled() -> bool {
  true
}

/// Response for [CreateDevice].
#[typeshare]
pub type CreateDeviceResponse = DeviceRecord;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/UpdateDevice",
  description = "Update a device",
  request_body(content = UpdateDevice),
  responses(
    (status = 200, description = "The updated device", body = UpdateDeviceResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn update_device() {}

/// Update a device. Response: [UpdateDeviceResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(UpdateDeviceResponse)]
#[error(mogh_error::Error)]
pub struct UpdateDevice {
  /// The device ID
  pub id: DeviceId,
  /// The name of the device
  pub name: Option<String>,
  /// The public key of the device
  pub public_key: Option<String>,
  /// Whether the device is enabled / has access.
  pub enabled: Option<bool>,
}

/// Response for [UpdateDevice].
#[typeshare]
pub type UpdateDeviceResponse = DeviceRecord;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/DeleteDevice",
  description = "Delete a device",
  request_body(content = DeleteDevice),
  responses(
    (status = 200, description = "The deleted device", body = DeleteDeviceResponse),
    (status = 404, description = "Device not found", body = mogh_error::Serror),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn delete_device() {}

/// Delete a device. Response: [DeleteDeviceResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
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

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/BatchDeleteDevices",
  description = "Batch delete devices",
  request_body(content = BatchDeleteDevices),
  responses(
    (status = 200, description = "The deleted devices", body = BatchDeleteDevicesResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn batch_delete_devices() {}

/// Batch delete devices. Response: [BatchDeleteDevicesResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(BatchDeleteDevicesResponse)]
#[error(mogh_error::Error)]
pub struct BatchDeleteDevices {
  /// The onboarding_key ID
  pub ids: Vec<DeviceId>,
}

/// Response for [BatchDeleteDevices].
#[typeshare]
pub type BatchDeleteDevicesResponse = Vec<DeviceRecord>;
