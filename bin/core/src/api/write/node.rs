use anyhow::{Context as _, anyhow};
use axum::http::StatusCode;
use cicada_client::api::write::node::{CreateNode, UpdateNode};
use resolver_api::Resolve;
use serror::AddStatusCodeError;

use crate::{api::write::WriteArgs, db::DB};

impl Resolve<WriteArgs> for CreateNode {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    let data = serde_json::to_string(&self)
      .context("Failed to serialize Node content")?;
    DB.query(format!("fn::create_node({data})"))
      .await
      .context("Failed to create node on database")?
      .take::<Option<_>>(0)
      .context("Failed to create node on database")?
      .context("No creation result")
      .map_err(Into::into)
  }
}

impl Resolve<WriteArgs> for UpdateNode {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    if self.id.0 == 1 {
      return Err(
        anyhow!("Cannot update root node (ino: 1)")
          .status_code(StatusCode::BAD_REQUEST),
      );
    }
    let update = serde_json::to_string(&self)
      .context("Failed to serialize MERGE update")?;
    DB.query(format!(
      r#"UPDATE type::record("Node", $id) MERGE {update}"#
    ))
    .bind(("id", self.id.0))
    .await
    .context("Failed to update Node on database")?
    .take::<Option<_>>(0)?
    .context("Failed to update Node on database: No update result")
    .map_err(Into::into)
  }
}
