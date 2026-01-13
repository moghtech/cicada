use cicada_client::{
  api::write::node::*, entities::node::NodeRecord,
};
use resolver_api::Resolve;

use crate::{api::write::WriteArgs, db::query};

#[allow(unused)]
#[utoipa::path(
  post,
  path = "/write/CreateNode",
  description = "Create a new node",
  request_body(content = CreateNode),
  responses(
    (status = 200, description = "The created node", body = NodeRecord),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn create_node() {}

impl Resolve<WriteArgs> for CreateNode {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::node::create_node(self).await.map_err(Into::into)
  }
}

//

#[allow(unused)]
#[utoipa::path(
  post,
  path = "/write/UpdateNode",
  description = "Update a node",
  request_body(content = UpdateNode),
  responses(
    (status = 200, description = "The updated node", body = NodeRecord),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn update_node() {}

impl Resolve<WriteArgs> for UpdateNode {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::node::update_node(self).await.map_err(Into::into)
  }
}

//

#[allow(unused)]
#[utoipa::path(
  post,
  path = "/write/DeleteNode",
  description = "Delete a node",
  request_body(content = DeleteNode),
  responses(
    (status = 200, description = "The deleted nodes", body = DeleteNodeResponse),
    (status = 404, description = "Node not found", body = mogh_error::Serror),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn delete_node() {}

impl Resolve<WriteArgs> for DeleteNode {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::node::delete_node(self.id.0, self.move_children).await
  }
}

//

#[allow(unused)]
#[utoipa::path(
  post,
  path = "/write/BatchDeleteNodes",
  description = "Batch delete many files / folders recursively.",
  request_body(content = BatchDeleteNodes),
  responses(
    (status = 200, description = "The deleted files / folders", body = BatchDeleteNodesResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn batch_delete_nodes() {}

impl Resolve<WriteArgs> for BatchDeleteNodes {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::node::batch_delete_nodes(self.ids)
      .await
      .map_err(Into::into)
  }
}
