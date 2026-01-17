use anyhow::Context;
use cicada_client::{
  api::write::node::*, entities::node::NodeEntity,
};
use resolver_api::Resolve;

use crate::{
  api::write::WriteArgs,
  db::query::{self, node::CreateNodeQuery},
  encryption::{
    decrypt_node, decrypt_nodes, encrypt_data, rotate_encryption_key,
    rotate_envelope_key,
  },
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
    let node = query::node::create_node(CreateNodeQuery {
      filesystem: self.filesystem,
      parent: self.parent,
      name: self.name,
      kind: self.kind,
    })
    .await?;
    let node = if let Some(data) = self.data {
      let encryption_key_id = if let Some(id) = self.encryption_key {
        id
      } else {
        query::encryption_key::list_all_encryption_keys()
          .await?
          .pop()
          .context("No encryption keys")?
          .id
      };
      let data = encrypt_data(
        encryption_key_id.0,
        data.as_bytes(),
        &node.id.0,
      )
      .await?;
      query::node::update_node_data(node.id, Some(data)).await?
    } else {
      node
    };
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
    let encryption_key = if let Some(id) = self.encryption_key {
      id
    } else if let Some(id) = query::node::get_node(&self.id.0)
      .await?
      .data
      .map(|data| data.encryption_key)
    {
      id
    } else {
      query::encryption_key::list_all_encryption_keys()
        .await?
        .pop()
        .context("No encryption keys")?
        .id
    };
    let data = encrypt_data(
      encryption_key.0,
      self.data.as_bytes(),
      &self.id.0,
    )
    .await?;
    let node =
      query::node::update_node_data(self.id, data.into()).await?;
    decrypt_node(node).await.map_err(Into::into)
  }
}

//

#[allow(unused)]
#[utoipa::path(
  post,
  path = "/write/UpdateNodeEncryptionKey",
  description = "Update a node's data",
  request_body(content = UpdateNode),
  responses(
    (status = 200, description = "The updated node", body = UpdateNodeEncryptionKeyResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn update_node_encryption_key() {}

impl Resolve<WriteArgs> for UpdateNodeEncryptionKey {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    let node = query::node::get_node(&self.id.0).await?;
    // No-op if node has no data.
    let Some(data) = node.data else {
      return Ok(NodeEntity {
        id: node.id,
        filesystem: node.filesystem,
        inode: node.inode,
        parent: node.parent,
        name: node.name,
        kind: node.kind,
        data: None,
        missing_key: None,
        created_at: node.created_at,
        updated_at: node.updated_at,
      });
    };
    // Re encrypt the envelope keys with new master key
    let data =
      rotate_encryption_key(data, &node.id.0, self.encryption_key.0)
        .await?;
    let node =
      query::node::update_node_data(self.id, data.into()).await?;
    decrypt_node(node).await.map_err(Into::into)
  }
}

//

#[allow(unused)]
#[utoipa::path(
  post,
  path = "/write/RotateNodeEnvelopeKey",
  description = "Update a node's data",
  request_body(content = UpdateNode),
  responses(
    (status = 200, description = "The updated node", body = RotateNodeEnvelopeKeyResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn rotate_node_envelope_key() {}

impl Resolve<WriteArgs> for RotateNodeEnvelopeKey {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    let node = query::node::get_node(&self.id.0).await?;
    // No-op if node has no data.
    let Some(data) = node.data else {
      return Ok(NodeEntity {
        id: node.id,
        filesystem: node.filesystem,
        inode: node.inode,
        parent: node.parent,
        name: node.name,
        kind: node.kind,
        data: None,
        missing_key: None,
        created_at: node.created_at,
        updated_at: node.updated_at,
      });
    };
    // Re encrypt data with new envelope key
    let data = rotate_envelope_key(data, &node.id.0).await?;
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
