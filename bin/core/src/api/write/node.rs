use cicada_client::{
  api::write::{
    BatchDeleteNodes, CreateNode, DeleteNode, RotateNodeEnvelopeKey,
    UpdateNode, UpdateNodeData, UpdateNodeEncryptionKey,
  },
  entities::node::NodeKind,
};
use futures_util::{StreamExt, stream::FuturesUnordered};
use mogh_error::anyhow::Context as _;
use mogh_resolver::Resolve;

use crate::{
  api::write::WriteArgs,
  db::query::{self, node::CreateNodeQuery},
  encryption::{
    decrypt_node, decrypt_nodes, encrypt_data, rotate_encryption_key,
    rotate_envelope_key,
  },
  permission::ensure_client_filesystem_permission,
};

impl Resolve<WriteArgs> for CreateNode {
  async fn resolve(
    self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    ensure_client_filesystem_permission(
      client,
      self.filesystem.clone(),
      true,
    )
    .await?;
    let node = query::node::create_node(CreateNodeQuery {
      filesystem: self.filesystem,
      parent: self.parent,
      name: self.name,
      perm: self.perm,
      kind: self.kind,
      interpolation: self.interpolation,
    })
    .await?;
    let node = if let NodeKind::File = node.kind {
      let data = self.data.unwrap_or_default();
      let encryption_key_id = if let Some(id) = self.encryption_key {
        id
      } else if let Some(id) =
        query::filesystem::get_filesystem(node.filesystem.0)
          .await?
          .encryption_key
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
        encryption_key_id.0,
        data.as_bytes(),
        &node.id.0,
      )
      .await?;
      query::node::update_node_data(node.id, Some(data)).await?
    } else {
      node
    };
    decrypt_node(node, self.interpolated).await
  }
}

//

impl Resolve<WriteArgs> for UpdateNode {
  async fn resolve(
    mut self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    let node =
      query::node::get_node_list_item(self.id.0.clone()).await?;
    ensure_client_filesystem_permission(
      client,
      node.filesystem,
      true,
    )
    .await?;
    let interpolated = self.interpolated.unwrap_or_default();
    // This isn't a field on database, set to None to stop serialization.
    self.interpolated = None;
    let node = query::node::update_node(self).await?;
    decrypt_node(node, interpolated).await
  }
}

//

impl Resolve<WriteArgs> for UpdateNodeData {
  async fn resolve(
    self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    let node =
      query::node::get_node_list_item(self.id.0.clone()).await?;
    ensure_client_filesystem_permission(
      client,
      node.filesystem,
      true,
    )
    .await?;
    let encryption_key = if let Some(id) = self.encryption_key {
      id
    } else if let Some(id) = node.encryption_key {
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
    decrypt_node(node, self.interpolated).await
  }
}

//

impl Resolve<WriteArgs> for UpdateNodeEncryptionKey {
  async fn resolve(
    self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    let node = query::node::get_node(&self.id.0).await?;
    ensure_client_filesystem_permission(
      client,
      node.filesystem.clone(),
      true,
    )
    .await?;
    // No-op if node has no data.
    let Some(data) = node.data else {
      return Ok(node.into_entity(None, None, false));
    };
    // Re encrypt the envelope keys with new master key
    let data =
      rotate_encryption_key(data, &node.id.0, self.encryption_key.0)
        .await?;
    let node =
      query::node::update_node_data(self.id, data.into()).await?;
    decrypt_node(node, self.interpolated).await
  }
}

//

impl Resolve<WriteArgs> for RotateNodeEnvelopeKey {
  async fn resolve(
    self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    let node = query::node::get_node(&self.id.0).await?;
    ensure_client_filesystem_permission(
      client,
      node.filesystem.clone(),
      true,
    )
    .await?;
    // No-op if node has no data.
    let Some(data) = node.data else {
      return Ok(node.into_entity(None, None, false));
    };
    // Re encrypt data with new envelope key
    let data = rotate_envelope_key(data, &node.id.0).await?;
    let node =
      query::node::update_node_data(self.id, data.into()).await?;
    decrypt_node(node, self.interpolated).await
  }
}

//

impl Resolve<WriteArgs> for DeleteNode {
  async fn resolve(
    self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    let node =
      query::node::get_node_list_item(self.id.0.clone()).await?;
    ensure_client_filesystem_permission(
      client,
      node.filesystem,
      true,
    )
    .await?;
    let deleted =
      query::node::delete_node(self.id.0, self.move_children).await?;
    Ok(decrypt_nodes(deleted, self.interpolated).await)
  }
}

//

impl Resolve<WriteArgs> for BatchDeleteNodes {
  async fn resolve(
    self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    let ids = if client.is_admin_user() {
      self.ids
    } else {
      // filter out any ids client doesn't
      // have necessary access to
      self
        .ids
        .into_iter()
        .map(|id| async {
          let node =
            query::node::get_node_list_item(id.0.clone()).await?;
          ensure_client_filesystem_permission(
            client,
            node.filesystem,
            true,
          )
          .await?;
          Result::<_, mogh_error::Error>::Ok(id)
        })
        .collect::<FuturesUnordered<_>>()
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    };
    if ids.is_empty() {
      return Ok(Vec::new());
    }
    let deleted = query::node::batch_delete_nodes(ids).await?;
    Ok(decrypt_nodes(deleted, self.interpolated).await)
  }
}
