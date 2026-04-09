use cicada_client::api::read::filesystem::{
  GetFilesystem, ListFilesystems,
};
use mogh_resolver::Resolve;

use crate::{
  api::read::ReadArgs,
  db::query,
  permission::{
    ensure_client_filesystem_permission, list_filesystems_for_client,
  },
};

impl Resolve<ReadArgs> for ListFilesystems {
  async fn resolve(
    self,
    ReadArgs { client }: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    list_filesystems_for_client(client).await
  }
}

impl Resolve<ReadArgs> for GetFilesystem {
  async fn resolve(
    self,
    ReadArgs { client }: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    let filesystem =
      query::filesystem::get_filesystem(self.id).await?;
    ensure_client_filesystem_permission(
      client,
      filesystem.id.clone(),
      false,
    )
    .await?;
    Ok(filesystem)
  }
}
