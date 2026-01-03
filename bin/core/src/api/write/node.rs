use anyhow::Context as _;
use cicada_client::{
  api::write::node::{CreateNode, UpdateNode},
  entities::node::NodeRecord,
};
use resolver_api::Resolve;

use crate::{api::write::WriteArgs, db::DB};

#[utoipa::path(
  post,
  path = "/write/CreateNode",
  description = "Create a new node",
  request_body(content = CreateNode),
  responses(
    (status = 200, description = "The created node", body = NodeRecord),
    (status = 500, description = "Request failed", body = serror::Serror)
  ),
)]
pub async fn create_node(
  body: CreateNode,
) -> serror::Result<NodeRecord> {
  // let data = serde_json::to_string(&body)
  //   .context("Failed to serialize Node content")?;
  // DB.query(format!("fn::create_node({data})"))
  //   .await
  //   .context("Failed to create node on database")?
  //   .take::<Option<_>>(0)
  //   .context("Failed to create node on database")?
  //   .context("No creation result")
  //   .map_err(Into::into)
  DB.create("Node")
    .content(body)
    .await
    .context("Failed to create Node on database")?
    .context("Failed to create Node on database: No creation result")
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
    (status = 200, description = "The updated node", body = NodeRecord),
    (status = 500, description = "Request failed", body = serror::Serror)
  ),
)]
pub async fn update_node(
  body: UpdateNode,
) -> serror::Result<NodeRecord> {
  // let update = serde_json::to_string(&body)
  //   .context("Failed to serialize MERGE update")?;
  // DB.query(format!(r#"UPDATE $id MERGE {update}"#))
  //   .bind(("id", body.id))
  //   .await
  //   .context("Failed to update Node on database")?
  //   .take::<Option<_>>(0)?
  //   .context("Failed to update Node on database: No update result")
  //   .map_err(Into::into)
  DB.update(body.id.as_record_id())
    .merge(serde_json::to_value(body)?)
    .await
    .context("Failed to update Node on database")?
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
