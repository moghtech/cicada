use cicada_client::api::read::node::{
  FindNode, FindNodeWithPath, GetNode, ListNodes,
};
use mogh_resolver::Resolve;

use crate::{
  api::read::ReadArgs, db::query, encryption::decrypt_node,
};

impl Resolve<ReadArgs> for ListNodes {
  async fn resolve(
    self,
    _: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::node::list_nodes(self.filesystem, self.parent).await
  }
}

impl Resolve<ReadArgs> for GetNode {
  async fn resolve(
    self,
    _: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    let node = query::node::get_node(&self.id.0).await?;
    decrypt_node(node, self.interpolated).await
  }
}

impl Resolve<ReadArgs> for FindNode {
  async fn resolve(
    self,
    _: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    let interpolated = self.interpolated;
    let node = query::node::find_node(self).await?;
    decrypt_node(node, interpolated).await
  }
}

impl Resolve<ReadArgs> for FindNodeWithPath {
  async fn resolve(
    self,
    _: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    let interpolated = self.interpolated;
    let node = query::node::find_node_with_path(self).await?;
    decrypt_node(node, interpolated).await
  }
}
