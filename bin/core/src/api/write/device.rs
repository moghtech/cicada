use cicada_client::api::write::device::*;
use mogh_resolver::Resolve;

use crate::{api::write::WriteArgs, db::query};

impl Resolve<WriteArgs> for CreateDevice {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::device::create_device(self).await
  }
}

//

impl Resolve<WriteArgs> for UpdateDevice {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::device::update_device(self).await
  }
}

//

impl Resolve<WriteArgs> for DeleteDevice {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::device::delete_device(self.id.0).await
  }
}

impl Resolve<WriteArgs> for BatchDeleteDevices {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::device::batch_delete_devices(self.ids).await
  }
}
