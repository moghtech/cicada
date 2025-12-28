use anyhow::Context;
use cicada_client::api::read::filesystem::ListFilesystems;
use resolver_api::Resolve;

use crate::{api::read::ReadArgs, db::DB};

impl Resolve<ReadArgs> for ListFilesystems {
  async fn resolve(
    self,
    _: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    DB.query("SELECT record::id(id) as id, * FROM Filesystem")
      .await
      .context("Failed to query for filesystems")?
      .take(0)
      .map_err(Into::into)
  }
}
