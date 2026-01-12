use derive_empty_traits::EmptyTraits;
use resolver_api::Resolve;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
  api::read::CicadaReadRequest, entities::device::DeviceRecord,
};

//

/// List devices. Response: [ListDevicesResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, Resolve, EmptyTraits,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaReadRequest)]
#[response(ListDevicesResponse)]
#[error(mogh_error::Error)]
pub struct ListDevices {}

/// Response for [ListDevices].
#[typeshare]
pub type ListDevicesResponse = Vec<DeviceRecord>;
