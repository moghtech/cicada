use anyhow::Context as _;
use axum::http::StatusCode;
use cicada_client::{
  api::write::filesystem::{CreateFilesystem, UpdateFilesystem},
  entities::filesystem::FilesystemRecord,
};
use mogh_error::AddStatusCode as _;

use crate::db::DB;

pub async fn list_all_filesystems()
-> anyhow::Result<Vec<FilesystemRecord>> {
  DB.select("Filesystem")
    .await
    .context("Failed to query for Filesystems")
}

pub async fn create_filesystem(
  body: CreateFilesystem,
) -> anyhow::Result<FilesystemRecord> {
  DB.create("Filesystem")
    .content(body)
    .await
    .context("Failed to create Filesystem on database")?
    .context(
      "Failed to create Filesystem on database: No creation result",
    )
}

pub async fn update_filesystem(
  body: UpdateFilesystem,
) -> anyhow::Result<FilesystemRecord> {
  DB.update(body.id.as_record_id())
    .merge(serde_json::to_value(body)?)
    .await
    .context("Failed to update Filesystem on database")?
    .context(
      "Failed to update Filesystem on database: No update result",
    )
}

pub async fn delete_filesystem(
  id: String,
) -> mogh_error::Result<FilesystemRecord> {
  DB.query("DELETE Node WHERE filesystem = $filesystem RETURN NONE;")
    .bind(("Filesystem", id.clone()))
    .await
    .context("Failed to delete Filesystem nodes")?;
  DB.delete(("Filesystem", id))
    .await?
    .context("No Filesystem matching given ID")
    .status_code(StatusCode::NOT_FOUND)
}
