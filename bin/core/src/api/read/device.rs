use cicada_client::{
  api::read::device::ListDevices, entities::device::DeviceRecord,
};
use resolver_api::Resolve;

use crate::{
  api::read::ReadArgs, db::query::device::list_all_devices,
};

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
    list_all_devices().await.map_err(Into::into)
  }
}
