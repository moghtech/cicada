use mogh_resolver::Resolve;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
  api::read::CicadaReadRequest,
  entities::device::{DeviceId, DeviceRecord},
};

//

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

/// Get a device. Response: [DeviceRecord].
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
