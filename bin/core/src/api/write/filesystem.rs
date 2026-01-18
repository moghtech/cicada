use cicada_client::api::write::filesystem::*;
use mogh_resolver::Resolve;

use crate::{api::write::WriteArgs, db::query};

#[allow(unused)]
#[utoipa::path(
  post,
  path = "/write/CreateFilesystem",
  description = "Create a new filesystem",
  request_body(content = CreateFilesystem),
  responses(
    (status = 200, description = "The created filesystem", body = CreateFilesystemResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn create_filesystem() {}

impl Resolve<WriteArgs> for CreateFilesystem {
  async fn resolve(
    self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    let _user = client.as_user()?;
    query::filesystem::create_filesystem(self).await
  }
}

//

#[allow(unused)]
#[utoipa::path(
  post,
  path = "/write/UpdateFilesystem",
  description = "Update a filesystem",
  request_body(content = UpdateFilesystem),
  responses(
    (status = 200, description = "The updated filesystem", body = UpdateFilesystemResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn update_filesystem() {}

impl Resolve<WriteArgs> for UpdateFilesystem {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::filesystem::update_filesystem(self).await
  }
}

//

#[allow(unused)]
#[utoipa::path(
  post,
  path = "/write/DeleteFilesystem",
  description = "Delete a filesystem",
  request_body(content = DeleteFilesystem),
  responses(
    (status = 200, description = "The deleted filesystem", body = DeleteFilesystemResponse),
    (status = 404, description = "Filesystem not found", body = mogh_error::Serror),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn delete_filesystem() {}

impl Resolve<WriteArgs> for DeleteFilesystem {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::filesystem::delete_filesystem(self.id.0).await
  }
}
