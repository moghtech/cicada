use cicada_client::api::write::{
  BatchDeleteFilesystems, CreateFilesystem, DeleteFilesystem,
  UpdateFilesystem,
};
use futures_util::{StreamExt as _, stream::FuturesUnordered};
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
    ensure_client_filesystem_permission(
      client,
      self.id.clone(),
      true,
    )
    .await?;
    query::filesystem::delete_filesystem(self.id).await
  }
}

//

impl Resolve<WriteArgs> for BatchDeleteFilesystems {
  async fn resolve(
    self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    let ids = if client.is_admin_user() {
      self.ids
    } else {
      // filter out any ids client doesn't
      // have necessary access to
      self
        .ids
        .into_iter()
        .map(|id| async {
          ensure_client_filesystem_permission(
            client,
            id.clone(),
            true,
          )
          .await?;
          Result::<_, mogh_error::Error>::Ok(id)
        })
        .collect::<FuturesUnordered<_>>()
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    };
    query::filesystem::batch_delete_filesystems(ids).await
  }
}
