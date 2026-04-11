use axum::http::StatusCode;
use cicada_client::{
  api::write::{CreateFilesystem, UpdateFilesystem},
  entities::filesystem::{FilesystemId, FilesystemRecord},
};
use mogh_error::AddStatusCode as _;
use mogh_error::anyhow::Context as _;

use crate::db::DB;

pub async fn list_all_filesystems()
-> mogh_error::Result<Vec<FilesystemRecord>> {
  DB.select("Filesystem")
    .await
    .context("Failed to query for Filesystems")
    .map_err(Into::into)
}

pub async fn get_filesystem(
  id_or_name: String,
) -> mogh_error::Result<FilesystemRecord> {
  DB.query(
    "
SELECT * FROM ONLY Filesystem
WHERE id = $id OR name = $name",
  )
  .bind(("id", FilesystemId(id_or_name.clone())))
  .bind(("name", id_or_name))
  .await
  .context("Failed to query database")?
  .take::<Option<FilesystemRecord>>(0)
  .context("Failed to get query result")?
  .context("Failed to find filesystem with given parameters.")
  .status_code(StatusCode::NOT_FOUND)
}

pub async fn create_filesystem(
  body: CreateFilesystem,
) -> mogh_error::Result<FilesystemRecord> {
  DB.create("Filesystem")
    .content(body)
    .await
    .context("Failed to create Filesystem on database")?
    .context(
      "Failed to create Filesystem on database: No creation result",
    )
    .map_err(Into::into)
}

pub async fn update_filesystem(
  body: UpdateFilesystem,
) -> mogh_error::Result<FilesystemRecord> {
  DB.query("UPDATE $id MERGE fn::object_strip_none($body);")
    .bind(("id", body.id.clone()))
    .bind(("body", body))
    .await
    .context("Failed to query database")?
    .take::<Option<FilesystemRecord>>(0)
    .context("Failed to get query result")?
    .context("Failed to find filesystem with given parameters.")
    .status_code(StatusCode::NOT_FOUND)
}

pub async fn delete_filesystem(
  id: FilesystemId,
) -> mogh_error::Result<FilesystemRecord> {
  DB.query("DELETE Node WHERE filesystem = $filesystem RETURN NONE;")
    .bind(("Filesystem", id.clone()))
    .await
    .context("Failed to delete Filesystem nodes")?;
  DB.delete(id.as_record_id())
    .await?
    .context("No Filesystem matching given ID")
    .status_code(StatusCode::NOT_FOUND)
}
