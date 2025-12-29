use anyhow::Context;
use axum::http::StatusCode;
use cicada_client::{
  api::read::node::{FindNode, GetNode, ListNodes},
  entities::{filesystem::FilesystemId, node::NodeRecord},
};
use resolver_api::Resolve;
use serror::AddStatusCode;

use crate::{api::read::ReadArgs, db::DB};

impl Resolve<ReadArgs> for ListNodes {
  async fn resolve(
    self,
    _: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    DB.query("SELECT id, filesystem, parent, name, kind FROM Node WHERE filesystem = $filesystem AND parent = $parent")
      .bind(("filesystem", FilesystemId(self.filesystem)))
      .bind(("parent", self.parent))
      .await
      .context("Failed to query for nodes")?
      .take(0)
      .map_err(Into::into)
  }
}

impl Resolve<ReadArgs> for GetNode {
  async fn resolve(
    self,
    _: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    DB.select(("Node", self.id as i64))
      .await
      .context("Failed to find node with given id.")?
      .context("Failed to find node with given id.")
      .status_code(StatusCode::NOT_FOUND)
  }
}

impl Resolve<ReadArgs> for FindNode {
  async fn resolve(
    self,
    _: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    DB.query(
      "SELECT * FROM Node WHERE filesystem = $filesystem AND parent = $parent AND name = $name",
    )
    .bind(("filesystem", FilesystemId(self.filesystem)))
    .bind(("parent", self.parent))
    .bind(("name", self.name))
    .await?
    .take::<Option<NodeRecord>>(0)?
    .context("Failed to find Node with given parent and name.")
    .status_code(StatusCode::NOT_FOUND)
  }
}
