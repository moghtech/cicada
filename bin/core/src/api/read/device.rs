use cicada_client::api::read::device::{GetDevice, ListDevices};
use mogh_resolver::Resolve;

use crate::{api::read::ReadArgs, db::query};

impl Resolve<ReadArgs> for ListDevices {
  async fn resolve(
    self,
    _: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::device::list_all_devices().await
  }
}

impl Resolve<ReadArgs> for GetDevice {
  async fn resolve(
    self,
    _: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::device::get_device(&self.id.0).await
  }
}
