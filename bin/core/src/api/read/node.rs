use anyhow::Context;
use cicada_client::{
  api::read::node::{GetNode, ListNodes},
  entities::node::NodeRecord,
};
use resolver_api::Resolve;

use crate::{api::read::ReadArgs, db::DB};

impl Resolve<ReadArgs> for ListNodes {
  async fn resolve(
    self,
    _: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    DB.select("Node")
      .await
      .context("Failed to query for nodes")
      .map_err(Into::into)
  }
}

impl Resolve<ReadArgs> for GetNode {
  async fn resolve(
    self,
    _: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    DB.query("SELECT * FROM Node WHERE ino = $ino")
      .bind(("ino", self.ino))
      .await?
      .take::<Option<NodeRecord>>(0)?
      .context("Failed to find node with given inode.")
      .map_err(Into::into)
  }
}
