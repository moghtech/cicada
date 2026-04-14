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
  db::query::{
    self, checkpoint::CreateCheckpointQuery, node::CreateNodeQuery,
  },
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
      checkpointing: self.checkpointing,
      interpolation: self.interpolation,
    })
    .await?;
    let node = if let NodeKind::File = node.kind {
      let data = self.data.unwrap_or_default();

      let filesystem =
        query::filesystem::get_filesystem(node.filesystem.0.clone())
          .await?;

      let encryption_key = if let Some(id) =
        self.encryption_key.or(filesystem.encryption_key)
      {
        id
      } else {
        // Takes the first available encryption key
        query::encryption_key::list_all_encryption_keys()
          .await?
          .pop()
          .context("No encryption keys")?
          .id
      };

      let data =
        encrypt_data(&encryption_key.0, data.as_bytes(), &node.id.0)
          .await?;

      let checkpoint = self
        .checkpoint
        .unwrap_or_else(|| filesystem.checkpointing.enabled())
        // Doing like this only clones when necessary
        .then(|| {
          (node.id.clone(), encryption_key.clone(), data.clone())
        });

      let (node, _) = tokio::try_join!(
        query::node::update_node_data(node.id, encryption_key, data),
        async {
          if let Some((node, encryption_key, data)) = checkpoint {
            query::checkpoint::create_checkpoint(
              CreateCheckpointQuery {
                node,
                name: self
                  .checkpoint_name
                  .unwrap_or_else(|| String::from("Create file"))
                  .into(),
                description: self.checkpoint_description,
                encryption_key,
                data,
              },
            )
            .await
            .map(|_| ())
          } else {
            Ok(())
          }
        },
      )?;

      node
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
      node.filesystem.clone(),
      true,
    )
    .await?;

    let filesystem =
      query::filesystem::get_filesystem(node.filesystem.0.clone())
        .await?;

    let encryption_key = if let Some(id) = self
      .encryption_key
      .or(node.encryption_key)
      .or(filesystem.encryption_key)
    {
      id
    } else {
      // Takes the first available encryption key
      query::encryption_key::list_all_encryption_keys()
        .await?
        .pop()
        .context("No encryption keys")?
        .id
    };

    let data = encrypt_data(
      &encryption_key.0,
      self.data.as_bytes(),
      &self.id.0,
    )
    .await?;

    let checkpoint = self
      .checkpoint
      .unwrap_or_else(|| {
        node
          .checkpointing
          .maybe_inherit(filesystem.checkpointing)
          .enabled()
      })
      // Doing like this only clones when necessary
      .then(|| (encryption_key.clone(), data.clone()));

    let (node, _) = tokio::try_join!(
      query::node::update_node_data(self.id, encryption_key, data),
      async {
        if let Some((encryption_key, data)) = checkpoint {
          query::checkpoint::create_checkpoint(
            CreateCheckpointQuery {
              node: node.id,
              name: self.checkpoint_name,
              description: self.checkpoint_description,
              encryption_key,
              data,
            },
          )
          .await
          .map(|_| ())
        } else {
          Ok(())
        }
      },
    )?;

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
      return Ok(node.into_entity(None));
    };

    let encryption_key = node
      .encryption_key
      .context("Node has data but no encryption key")?;

    // Re encrypt the envelope keys with new master key
    let data = rotate_encryption_key(
      &encryption_key.0,
      data,
      &node.id.0,
      &self.encryption_key.0,
    )
    .await?;

    let node = query::node::update_node_data(
      self.id,
      self.encryption_key,
      data,
    )
    .await?;
  
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
      return Ok(node.into_entity(None));
    };
    let encryption_key = node
      .encryption_key
      .context("Node has data but no encryption key")?;
    // Re encrypt data with new envelope key
    let data =
      rotate_envelope_key(&encryption_key.0, data, &node.id.0)
        .await?;
    let node = query::node::update_node_data(
      self.id,
      encryption_key,
      data,
    )
    .await?;
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
