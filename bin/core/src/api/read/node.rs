use cicada_client::{
  api::read::node::{FindNode, GetNode, ListNodes},
  entities::node::{NodeListItem, NodeRecord},
};
use resolver_api::Resolve;

use crate::{api::read::ReadArgs, db::query};

#[allow(unused)]
#[utoipa::path(
  post,
  path = "/read/ListNodes",
  description = "List available folders and files",
  request_body(content = ListNodes),
  responses(
    (status = 200, description = "List of filesystem nodes", body = Vec<NodeListItem>),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn list_nodes() {}

impl Resolve<ReadArgs> for ListNodes {
  async fn resolve(
    self,
    _: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::node::list_nodes(self.filesystem, self.parent)
      .await
      .map_err(Into::into)
  }
}

#[allow(unused)]
#[utoipa::path(
  post,
  path = "/read/GetNode",
  description = "Get a folder or file by id",
  request_body(content = GetNode),
  responses(
    (status = 200, description = "The filesystem node", body = NodeRecord),
    (status = 404, description = "Failed to find node with given id", body = mogh_error::Serror),
    (status = 500, description = "Request failed", body = mogh_error::Serror),
  ),
)]
pub fn get_node() {}

impl Resolve<ReadArgs> for GetNode {
  async fn resolve(
    self,
    _: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::node::get_node(&self.id.0).await
  }
}

#[allow(unused)]
#[utoipa::path(
  post,
  path = "/read/FindNode",
  description = "Find a node by filesystem + inode OR filesystem + parent inode + name",
  request_body(content = FindNode),
  responses(
    (status = 200, description = "The filesystem node", body = NodeRecord),
    (status = 404, description = "Failed to find node with given parameters", body = mogh_error::Serror),
    (status = 500, description = "Request failed", body = mogh_error::Serror),
  ),
)]
pub fn find_node() {}

impl Resolve<ReadArgs> for FindNode {
  async fn resolve(
    self,
    _: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::node::find_node(self).await
  }
}
