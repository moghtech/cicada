use anyhow::Context;
use cicada_client::api::write::node::*;
use resolver_api::Resolve;

use crate::{
  api::write::WriteArgs,
  db::query::{self, node::CreateNodeQuery},
  encryption::{decrypt_node, decrypt_nodes, encrypt_data},
};

#[allow(unused)]
#[utoipa::path(
  post,
  path = "/write/CreateNode",
  description = "Create a new node",
  request_body(content = CreateNode),
  responses(
    (status = 200, description = "The created node", body = CreateNodeResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn create_node() {}

impl Resolve<WriteArgs> for CreateNode {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    let encryption_key =
      query::encryption_key::list_all_encryption_keys()
        .await?
        .pop()
        .context("No encryption key available")?;
    let data = if let Some(data) = self.data {
      encrypt_data(encryption_key, data.as_bytes()).await?.into()
    } else {
      None
    };
    let node = query::node::create_node(CreateNodeQuery {
      filesystem: self.filesystem,
      parent: self.parent,
      name: self.name,
      kind: self.kind,
      data,
    })
    .await?;
    decrypt_node(node).await.map_err(Into::into)
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
    (status = 200, description = "The updated node", body = UpdateNodeResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn update_node() {}

impl Resolve<WriteArgs> for UpdateNode {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    let node = query::node::update_node(self).await?;
    decrypt_node(node).await.map_err(Into::into)
  }
}

//

#[allow(unused)]
#[utoipa::path(
  post,
  path = "/write/UpdateNodeData",
  description = "Update a node's data",
  request_body(content = UpdateNode),
  responses(
    (status = 200, description = "The updated node", body = UpdateNodeDataResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn update_node_data() {}

impl Resolve<WriteArgs> for UpdateNodeData {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    let encryption_key =
      query::encryption_key::list_all_encryption_keys()
        .await?
        .pop()
        .context("No encryption key available")?;
    let data =
      encrypt_data(encryption_key, self.data.as_bytes()).await?;
    let node =
      query::node::update_node_data(self.id, data.into()).await?;
    decrypt_node(node).await.map_err(Into::into)
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
    let deleted =
      query::node::delete_node(self.id.0, self.move_children).await?;
    Ok(decrypt_nodes(deleted).await)
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
    let deleted = query::node::batch_delete_nodes(self.ids).await?;
    Ok(decrypt_nodes(deleted).await)
  }
}
