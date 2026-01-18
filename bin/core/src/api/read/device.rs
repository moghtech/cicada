use cicada_client::{
  api::read::device::{GetDevice, ListDevices},
  entities::device::DeviceRecord,
};
use mogh_resolver::Resolve;

use crate::{api::read::ReadArgs, db::query};

#[allow(unused)]
#[utoipa::path(
  post,
  path = "/read/ListDevices",
  description = "List devices",
  request_body(content = ListDevices),
  responses(
    (status = 200, description = "List of devices", body = Vec<DeviceRecord>),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn list_devices() {}

impl Resolve<ReadArgs> for ListDevices {
  async fn resolve(
    self,
    _: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::device::list_all_devices().await
  }
}

#[allow(unused)]
#[utoipa::path(
  post,
  path = "/read/GetDevice",
  description = "Get a device by id",
  request_body(content = GetDevice),
  responses(
    (status = 200, description = "The device", body = DeviceRecord),
    (status = 404, description = "Failed to find device with given id", body = mogh_error::Serror),
    (status = 500, description = "Request failed", body = mogh_error::Serror),
  ),
)]
pub fn get_device(body: GetDevice) {}

impl Resolve<ReadArgs> for GetDevice {
  async fn resolve(
    self,
    _: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::device::get_device(&self.id.0).await
  }
}
