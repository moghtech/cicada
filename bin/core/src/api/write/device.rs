use cicada_client::api::write::{
  BatchDeleteDevices, CreateDevice, DeleteDevice, UpdateDevice,
};
use mogh_resolver::Resolve;

use crate::{api::write::WriteArgs, db::query};

impl Resolve<WriteArgs> for CreateDevice {
  async fn resolve(
    self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    // Only allow onboarding keys and admin level users
    client.not_device()?;
    client.only_admin_users()?;
    query::device::create_device(self).await
  }
}

//

impl Resolve<WriteArgs> for UpdateDevice {
  async fn resolve(
    self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    client.admin_only()?;
    query::device::update_device(self).await
  }
}

//

impl Resolve<WriteArgs> for DeleteDevice {
  async fn resolve(
    self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    client.admin_only()?;
    query::device::delete_device(self.id.0).await
  }
}

impl Resolve<WriteArgs> for BatchDeleteDevices {
  async fn resolve(
    self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    client.admin_only()?;
    query::device::batch_delete_devices(self.ids).await
  }
}
