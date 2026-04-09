use std::collections::HashSet;

use axum::http::StatusCode;
use cicada_client::entities::filesystem::{
  FilesystemId, FilesystemRecord,
};
use mogh_error::{AddStatusCodeError as _, anyhow::Context as _};

use crate::{
  auth::middleware::Client,
  db::{
    DB,
    query::{
      filesystem::list_all_filesystems,
      policy::list_policies_for_client,
    },
  },
};

pub async fn list_filesystems_for_client(
  client: &Client,
) -> mogh_error::Result<Vec<FilesystemRecord>> {
  if let Client::User(user) = client
    && user.admin
  {
    return list_all_filesystems().await;
  }

  let filesystem_ids = list_policies_for_client(client)
    .await?
    .into_iter()
    .flat_map(|p| p.filesystems)
    .collect::<HashSet<_>>();

  DB.query("SELECT * FROM Filesystem WHERE id IN $ids")
    .bind(("ids", filesystem_ids))
    .await
    .context("Failed to query database for filesystems")?
    .take::<Vec<FilesystemRecord>>(0)
    .context("Failed to get filesystem query result")
    .map_err(Into::into)
}

pub async fn ensure_client_filesystem_permission(
  client: &Client,
  filesystem: FilesystemId,
  write_required: bool,
) -> mogh_error::Result<()> {
  if let Client::User(user) = client
    && user.admin
  {
    return Ok(());
  }

  let policies = list_policies_for_client(client).await?;

  // Check if any policy grants access to the whole filesystem
  if policies.iter().any(|policy| {
    let fs_access =
      policy.filesystems.iter().any(|fs| fs.0 == filesystem.0);
    if write_required {
      fs_access && policy.filesystem_write
    } else {
      fs_access
    }
  }) {
    return Ok(());
  }

  Err(
    mogh_error::anyhow::anyhow!("Permission denied")
      .status_code(StatusCode::FORBIDDEN),
  )
}
