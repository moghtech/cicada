use mogh_resolver::Resolve;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
  api::read::CicadaReadRequest,
  entities::device::{DeviceId, DeviceRecord},
};

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/read/ListDevices",
  description = "List devices.",
  request_body(content = ListDevices),
  responses(
    (status = 200, description = "List of devices", body = ListDevicesResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn list_devices() {}

/// List devices. Response: [ListDevicesResponse].
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaReadRequest)]
#[response(ListDevicesResponse)]
#[error(mogh_error::Error)]
pub struct ListDevices {}

/// Response for [ListDevices].
#[typeshare]
pub type ListDevicesResponse = Vec<DeviceRecord>;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/read/GetDevice",
  description = "Get a device by id.",
  request_body(content = GetDevice),
  responses(
    (status = 200, description = "The device", body = GetDeviceResponse),
    (status = 404, description = "Failed to find device with given id", body = mogh_error::Serror),
    (status = 500, description = "Request failed", body = mogh_error::Serror),
  ),
)]
pub fn get_device() {}

/// Get a device by id. Response: [DeviceRecord].
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaReadRequest)]
#[response(GetDeviceResponse)]
#[error(mogh_error::Error)]
pub struct GetDevice {
  /// The device id
  pub id: DeviceId,
}

/// Response for [GetDevice].
#[typeshare]
pub type GetDeviceResponse = DeviceRecord;
