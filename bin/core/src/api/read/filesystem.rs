use anyhow::Context;
use cicada_client::{
  api::read::filesystem::ListFilesystems,
  entities::filesystem::FilesystemRecord,
};
use resolver_api::Resolve;

use crate::{api::read::ReadArgs, db::DB};

#[utoipa::path(
  post,
  path = "/read/ListFilesystems",
  description = "List available filesystems",
  request_body(content = ListFilesystems),
  responses(
    (status = 200, body = Vec<FilesystemRecord>),
    (status = 500, description = "Failed to query database")
  ),
)]
pub async fn list_filesystems()
-> serror::Result<Vec<FilesystemRecord>> {
  DB.select("Filesystem")
    .await
    .context("Failed to query for filesystems")
    .map_err(Into::into)
}

impl Resolve<ReadArgs> for ListFilesystems {
  async fn resolve(
    self,
    _: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    list_filesystems().await
  }
}
