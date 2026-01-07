use cicada_client::{
  api::read::filesystem::ListFilesystems,
  entities::filesystem::FilesystemRecord,
};
use resolver_api::Resolve;

use crate::{
  api::read::ReadArgs, db::query::filesystem::list_all_filesystems,
};

#[allow(unused)]
#[utoipa::path(
  post,
  path = "/read/ListFilesystems",
  description = "List available filesystems",
  request_body(content = ListFilesystems),
  responses(
    (status = 200, description = "List of filesystems", body = Vec<FilesystemRecord>),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn list_filesystems() {}

impl Resolve<ReadArgs> for ListFilesystems {
  async fn resolve(
    self,
    _: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    list_all_filesystems().await.map_err(Into::into)
  }
}
