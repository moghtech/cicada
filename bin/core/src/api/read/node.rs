use anyhow::Context;
use axum::http::StatusCode;
use cicada_client::{
  api::read::node::{FindNode, GetNode, ListNodes},
  entities::{
    filesystem::FilesystemId,
    node::{NodeListItem, NodeRecord},
  },
};
use resolver_api::Resolve;
use serror::AddStatusCode;

use crate::{api::read::ReadArgs, db::DB};

#[utoipa::path(
  post,
  path = "/read/ListNodes",
  description = "List available folders and files",
  request_body(content = ListNodes),
  responses(
    (status = 200, description = "List of filesystem nodes", body = Vec<NodeListItem>),
    (status = 500, description = "Request failed", body = serror::Serror)
  ),
)]
pub async fn list_nodes(
  body: ListNodes,
) -> serror::Result<Vec<NodeListItem>> {
  DB.query(
    "
SELECT id, filesystem, parent, name, kind FROM Node 
WHERE ($filesystem IS NONE OR filesystem = $filesystem)
AND ($parent IS NONE OR parent = $parent)",
  )
  .bind(("filesystem", body.filesystem.map(FilesystemId)))
  .bind(("parent", body.parent))
  .await
  .context("Failed to query for nodes")?
  .take(0)
  .map_err(Into::into)
}

impl Resolve<ReadArgs> for ListNodes {
  async fn resolve(
    self,
    _: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    list_nodes(self).await
  }
}

#[utoipa::path(
  post,
  path = "/read/GetNode",
  description = "Get a folder or file by id",
  request_body(content = GetNode),
  responses(
    (status = 200, description = "The filesystem node", body = NodeRecord),
    (status = 404, description = "Failed to find node with given id", body = serror::Serror),
    (status = 500, description = "Request failed", body = serror::Serror),
  ),
)]
pub async fn get_node(body: GetNode) -> serror::Result<NodeRecord> {
  DB.select(("Node", body.id as i64))
    .await
    .context("Failed to find node with given id.")?
    .context("Failed to find node with given id.")
    .status_code(StatusCode::NOT_FOUND)
}

impl Resolve<ReadArgs> for GetNode {
  async fn resolve(
    self,
    _: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    get_node(self).await
  }
}

#[utoipa::path(
  post,
  path = "/read/FindNode",
  description = "Find a node by parent id and name",
  request_body(content = FindNode),
  responses(
    (status = 200, description = "The filesystem node", body = NodeRecord),
    (status = 404, description = "Failed to find node with given parent / name", body = serror::Serror),
    (status = 500, description = "Request failed", body = serror::Serror),
  ),
)]
pub async fn find_node(body: FindNode) -> serror::Result<NodeRecord> {
  DB.query(
      "SELECT * FROM Node WHERE filesystem = $filesystem AND parent = $parent AND name = $name",
    )
    .bind(("filesystem", FilesystemId(body.filesystem)))
    .bind(("parent", body.parent))
    .bind(("name", body.name))
    .await?
    .take::<Option<NodeRecord>>(0)?
    .context("Failed to find Node with given parent and name.")
    .status_code(StatusCode::NOT_FOUND)
}

impl Resolve<ReadArgs> for FindNode {
  async fn resolve(
    self,
    _: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    find_node(self).await
  }
}
