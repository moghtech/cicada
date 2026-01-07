use anyhow::Context as _;
use cicada_client::entities::filesystem::FilesystemRecord;

use crate::db::DB;

pub async fn list_all_filesystems()
-> anyhow::Result<Vec<FilesystemRecord>> {
  DB.select("Filesystem")
    .await
    .context("Failed to query for filesystems")
}
