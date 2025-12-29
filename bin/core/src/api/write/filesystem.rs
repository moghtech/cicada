use anyhow::Context as _;
use cicada_client::api::write::filesystem::{
  CreateFilesystem, UpdateFilesystem,
};
use resolver_api::Resolve;

use crate::{api::write::WriteArgs, db::DB};

impl Resolve<WriteArgs> for CreateFilesystem {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    DB.insert::<Vec<_>>("Filesystem")
      .content(self)
      .await
      .context("Failed to create filesystem on database")?
      .pop()
      .context(
        "Failed to create filesystem on database: No creation result",
      )
      .map_err(Into::into)
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
    .bind(("id", self.id))
    .await
    .context("Failed to update filesystem on database")?
    .take::<Option<_>>(0)?
    .context(
      "Failed to update filesystem on database: No update result",
    )
    .map_err(Into::into)
  }
}
