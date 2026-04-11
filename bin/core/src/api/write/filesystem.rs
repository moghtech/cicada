use cicada_client::api::write::{
  CreateFilesystem, DeleteFilesystem, UpdateFilesystem,
};
use mogh_resolver::Resolve;

use crate::{
  api::write::WriteArgs, db::query,
  permission::ensure_client_filesystem_permission,
};

impl Resolve<WriteArgs> for CreateFilesystem {
  async fn resolve(
    self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    client.admin_only()?;
    query::filesystem::create_filesystem(self).await
  }
}

//

impl Resolve<WriteArgs> for UpdateFilesystem {
  async fn resolve(
    self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    ensure_client_filesystem_permission(
      client,
      self.id.clone(),
      true,
    )
    .await?;
    query::filesystem::update_filesystem(self).await
  }
}

//

impl Resolve<WriteArgs> for DeleteFilesystem {
  async fn resolve(
    self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    client.admin_only()?;
    query::filesystem::delete_filesystem(self.id).await
  }
}
