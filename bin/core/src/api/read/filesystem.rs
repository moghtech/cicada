use cicada_client::api::read::filesystem::{
  GetFilesystem, ListFilesystems,
};
use mogh_resolver::Resolve;

use crate::{
  api::read::ReadArgs,
  db::query::{self, filesystem::list_all_filesystems},
};

impl Resolve<ReadArgs> for ListFilesystems {
  async fn resolve(
    self,
    _: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    list_all_filesystems().await
  }
}

impl Resolve<ReadArgs> for GetFilesystem {
  async fn resolve(
    self,
    _: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::filesystem::get_filesystem(self.id).await
  }
}
