use anyhow::Context as _;
use cicada_client::{
  api::write::filesystem::{CreateFilesystem, UpdateFilesystem},
  entities::filesystem::FilesystemRecord,
};
use resolver_api::Resolve;

use crate::{api::write::WriteArgs, db::DB};

impl Resolve<WriteArgs> for CreateFilesystem {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    let name = self.name.clone();
    DB.insert::<Vec<surrealdb::types::Value>>("Filesystem")
      .content(self)
      .await
      .context("Failed to create filesystem on database")?
      .pop()
      .context(
        "Failed to create filesystem on database: No creation result",
      )?;
    get_filesystem_by_name(name).await.map_err(Into::into)
  }
}

impl Resolve<WriteArgs> for UpdateFilesystem {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    let update = serde_json::to_string(&self)
      .context("Failed to serialize MERGE update")?;
    DB.query(format!(
      r#"UPDATE type::record("Filesystem", $id) MERGE {update}"#
    ))
    .bind(("id", self.id.clone()))
    .await
    .context("Failed to update filesystem on database")?
    .take::<Option<surrealdb::types::Value>>(0)?
    .context(
      "Failed to update filesystem on database: No update result",
    )?;
    get_filesystem(self.id).await.map_err(Into::into)
  }
}

async fn get_filesystem(
  id: String,
) -> anyhow::Result<FilesystemRecord> {
  DB.query(r#"SELECT record::id(id) as id, * FROM type::record("Filesystem", $id)"#)
      .bind(("id", id))
      .await
      .context("Failed to query for filesystems")?
      .take::<Option<_>>(0)
      .context(
        "Failed to update filesystem on database: Failed to query",
      )?
      .context("Failed to update filesystem on database: No result after update")
}

async fn get_filesystem_by_name(
  name: String,
) -> anyhow::Result<FilesystemRecord> {
  DB.query("SELECT record::id(id) as id, * FROM Filesystem WHERE name = $name")
      .bind(("name", name))
      .await
      .context("Failed to query for filesystems")?
      .take::<Option<_>>(0)
      .context(
        "Failed to update filesystem on database: Failed to query",
      )?
      .context("Failed to update filesystem on database: No result after update")
}
