use anyhow::{Context as _, anyhow};
use axum::http::StatusCode;
use cicada_client::{
  api::write::node::{CreateNode, UpdateNode},
  entities::node::NodeRecord,
};
use resolver_api::Resolve;
use serror::AddStatusCodeError;

use crate::{api::write::WriteArgs, db::DB};

#[utoipa::path(
  post,
  path = "/write/CreateNode",
  description = "Create a new node",
  request_body(content = CreateNode),
  responses(
    (status = 200, body = NodeRecord),
    (status = 500, description = "Request failed", body = serror::Serror)
  ),
)]
pub async fn create_node(
  body: CreateNode,
) -> serror::Result<NodeRecord> {
  let data = serde_json::to_string(&body)
    .context("Failed to serialize Node content")?;
  DB.query(format!("fn::create_node({data})"))
    .await
    .context("Failed to create node on database")?
    .take::<Option<_>>(0)
    .context("Failed to create node on database")?
    .context("No creation result")
    .map_err(Into::into)
}

impl Resolve<WriteArgs> for CreateNode {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    create_node(self).await
  }
}

#[utoipa::path(
  post,
  path = "/write/UpdateNode",
  description = "Update a node",
  request_body(content = UpdateNode),
  responses(
    (status = 200, body = NodeRecord),
    (status = 500, description = "Request failed", body = serror::Serror)
  ),
)]
pub async fn update_node(
  body: UpdateNode,
) -> serror::Result<NodeRecord> {
  if body.id.0 == 1 {
    return Err(
      anyhow!("Cannot update root node (ino: 1)")
        .status_code(StatusCode::BAD_REQUEST),
    );
  }
  let update = serde_json::to_string(&body)
    .context("Failed to serialize MERGE update")?;
  DB.query(format!(
    r#"UPDATE type::record("Node", $id) MERGE {update}"#
  ))
  .bind(("id", body.id.0))
  .await
  .context("Failed to update Node on database")?
  .take::<Option<_>>(0)?
  .context("Failed to update Node on database: No update result")
  .map_err(Into::into)
}

impl Resolve<WriteArgs> for UpdateNode {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    update_node(self).await
  }
}
