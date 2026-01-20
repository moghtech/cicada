use cicada_client::api::read::filesystem::ListFilesystems;
use mogh_resolver::Resolve;

use crate::{
  api::read::ReadArgs, db::query::filesystem::list_all_filesystems,
};

impl Resolve<ReadArgs> for ListFilesystems {
  async fn resolve(
    self,
    _: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    list_all_filesystems().await
  }
}
