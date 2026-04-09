use cicada_client::api::read::node::{
  FindNode, FindNodeWithPath, GetNode, ListNodes,
};
use mogh_resolver::Resolve;

use crate::{
  api::read::ReadArgs, db::query, encryption::decrypt_node,
  permission::ensure_client_filesystem_permission,
};

impl Resolve<ReadArgs> for ListNodes {
  async fn resolve(
    self,
    ReadArgs { client }: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    ensure_client_filesystem_permission(
      client,
      self.filesystem.clone(),
      false,
    )
    .await?;
    query::node::list_child_nodes(self.filesystem, self.parent).await
  }
}

impl Resolve<ReadArgs> for GetNode {
  async fn resolve(
    self,
    ReadArgs { client }: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    let node = query::node::get_node(&self.id.0).await?;
    // Don't know filesystem id for perm check until node query.
    ensure_client_filesystem_permission(
      client,
      node.filesystem.clone(),
      false,
    )
    .await?;
    decrypt_node(node, self.interpolated).await
  }
}

impl Resolve<ReadArgs> for FindNode {
  async fn resolve(
    self,
    ReadArgs { client }: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    ensure_client_filesystem_permission(
      client,
      self.filesystem.clone(),
      false,
    )
    .await?;
    let interpolated = self.interpolated;
    let node = query::node::find_node(self).await?;
    decrypt_node(node, interpolated).await
  }
}

impl Resolve<ReadArgs> for FindNodeWithPath {
  async fn resolve(
    self,
    ReadArgs { client }: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    ensure_client_filesystem_permission(
      client,
      self.filesystem.clone(),
      false,
    )
    .await?;
    let interpolated = self.interpolated;
    let node = query::node::find_node_with_path(self).await?;
    decrypt_node(node, interpolated).await
  }
}
