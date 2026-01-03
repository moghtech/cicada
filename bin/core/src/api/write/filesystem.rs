use anyhow::Context as _;
use axum::http::StatusCode;
use cicada_client::{
  api::write::filesystem::{
    CreateFilesystem, DeleteFilesystem, UpdateFilesystem,
  },
  entities::filesystem::FilesystemRecord,
};
use resolver_api::Resolve;
use serror::AddStatusCode;

use crate::{api::write::WriteArgs, db::DB};

#[utoipa::path(
  post,
  path = "/write/CreateFilesystem",
  description = "Create a new filesystem",
  request_body(content = CreateFilesystem),
  responses(
    (status = 200, description = "The created filesystem", body = FilesystemRecord),
    (status = 500, description = "Request failed", body = serror::Serror)
  ),
)]
pub async fn create_filesystem(
  body: CreateFilesystem,
) -> serror::Result<FilesystemRecord> {
  // DB.insert::<Vec<_>>("Filesystem")
  //   .content(body)
  //   .await
  //   .context("Failed to create filesystem on database")?
  //   .pop()
  //   .context(
  //     "Failed to create filesystem on database: No creation result",
  //   )
  //   .map_err(Into::into)
  DB.create("Filesystem")
    .content(body)
    .await
    .context("Failed to create Filesystem on database")?
    .context(
      "Failed to create Filesystem on database: No creation result",
    )
    .map_err(Into::into)
}

impl Resolve<WriteArgs> for CreateFilesystem {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    create_filesystem(self).await
  }
}

//

#[utoipa::path(
  post,
  path = "/write/UpdateFilesystem",
  description = "Update a filesystem",
  request_body(content = UpdateFilesystem),
  responses(
    (status = 200, description = "The updated filesystem", body = FilesystemRecord),
    (status = 500, description = "Request failed", body = serror::Serror)
  ),
)]
pub async fn update_filesystem(
  body: UpdateFilesystem,
) -> serror::Result<FilesystemRecord> {
  // let update = serde_json::to_string(&body)
  //   .context("Failed to serialize MERGE update")?;
  // DB.query(format!(
  //   r#"UPDATE type::record("Filesystem", $id) MERGE {update}"#
  // ))
  // .bind(("id", body.id))
  // .await
  // .context("Failed to update filesystem on database")?
  // .take::<Option<_>>(0)?
  // .context(
  //   "Failed to update filesystem on database: No update result",
  // )
  // .map_err(Into::into)
  DB.update(body.id.as_record_id())
    .merge(serde_json::to_value(body)?)
    .await
    .context("Failed to update Filesystem on database")?
    .context(
      "Failed to update Filesystem on database: No update result",
    )
    .map_err(Into::into)
}

impl Resolve<WriteArgs> for UpdateFilesystem {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    update_filesystem(self).await
  }
}

//

#[utoipa::path(
  post,
  path = "/write/DeleteFilesystem",
  description = "Delete a filesystem",
  request_body(content = DeleteFilesystem),
  responses(
    (status = 200, description = "The deleted filesystem", body = FilesystemRecord),
    (status = 404, description = "Filesystem not found", body = serror::Serror),
    (status = 500, description = "Request failed", body = serror::Serror)
  ),
)]
pub async fn delete_filesystem(
  body: DeleteFilesystem,
) -> serror::Result<FilesystemRecord> {
  DB.query("DELETE Node WHERE filesystem = $filesystem RETURN NONE;")
    .bind(("filesystem", body.id.clone()))
    .await
    .context("Failed to delete filesystem nodes")?;
  DB
    .delete(body.id.as_record_id())
    .await?
    .context("No filesystem matching given ID")
    .status_code(StatusCode::NOT_FOUND)
}

impl Resolve<WriteArgs> for DeleteFilesystem {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    delete_filesystem(self).await
  }
}
