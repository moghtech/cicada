use cicada_client::api::write::device::*;
use resolver_api::Resolve;

use crate::{api::write::WriteArgs, db::query};

#[allow(unused)]
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

impl Resolve<WriteArgs> for CreateDevice {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::device::create_device(self).await.map_err(Into::into)
  }
}

//

#[allow(unused)]
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

impl Resolve<WriteArgs> for UpdateDevice {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::device::update_device(self).await.map_err(Into::into)
  }
}

//

#[allow(unused)]
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

impl Resolve<WriteArgs> for DeleteDevice {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::device::delete_device(self.id.0).await
  }
}

#[allow(unused)]
#[utoipa::path(
  post,
  path = "/write/BatchDeleteDevices",
  description = "Batch delete devices",
  request_body(content = BatchDeleteDevices),
  responses(
    (status = 200, description = "Devices deleted", body = BatchDeleteDevicesResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn batch_delete_devices() {}

impl Resolve<WriteArgs> for BatchDeleteDevices {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::device::batch_delete_devices(self.ids).await?;
    Ok(BatchDeleteDevicesResponse {})
  }
}
