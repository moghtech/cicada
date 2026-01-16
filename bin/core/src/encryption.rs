use anyhow::{Context, anyhow};
use cicada_client::entities::{
  EncryptedData,
  encryption_key::EncryptionKeyRecord,
  node::{NodeEntity, NodeRecord},
};
use futures_util::{StreamExt as _, stream::FuturesOrdered};
use mogh_encryption::{
  BASE64URL, EnvelopeEncryptedData, xchacha20poly1305,
};

use crate::db::query;

pub async fn encrypt_data(
  encryption_key: EncryptionKeyRecord,
  data: &[u8],
) -> anyhow::Result<EncryptedData> {
  let master_key = encryption_key
    .key
    .context("Missing on disk encryption key")?;
  let master_key: [u8; 32] = BASE64URL
    .decode(master_key.as_bytes())
    .context("Invalid on disk encryption key")?
    .try_into()
    .map_err(|_| anyhow!("Invalid on disk encryption key"))?;
  let EnvelopeEncryptedData { key, data } =
    xchacha20poly1305::EncryptionProvider::default()
      .envelope_encrypt(data, master_key, &encryption_key.id.0)?;
  Ok(EncryptedData {
    encryption_key: encryption_key.id,
    key: key.data,
    key_nonce: key.nonce,
    data: data.data,
    data_nonce: data.nonce,
  })
}

pub async fn decrypt_data<T: TryFrom<Vec<u8>>>(
  data: EncryptedData,
) -> anyhow::Result<T>
where
  T::Error: Send + Sync + std::error::Error + 'static,
{
  let encryption_key =
    query::encryption_key::get_encryption_key(&data.encryption_key.0)
      .await
      .map_err(|e| e.error)?;
  let master_key = encryption_key
    .key
    .context("Missing on disk encryption key")?;
  let master_key: [u8; 32] = BASE64URL
    .decode(master_key.as_bytes())
    .context("Invalid on disk encryption key")?
    .try_into()
    .map_err(|_| anyhow!("Invalid on disk encryption key"))?;
  let data = EnvelopeEncryptedData {
    key: mogh_encryption::EncryptedData {
      data: data.key,
      nonce: data.key_nonce,
    },
    data: mogh_encryption::EncryptedData {
      data: data.data,
      nonce: data.data_nonce,
    },
  };
  let data = xchacha20poly1305::envelope_decrypt(
    &data,
    master_key,
    &encryption_key.id.0,
  )?;
  data.try_into().context("Failed to convert data from bytes")
}

pub async fn decrypt_node(
  node: NodeRecord,
) -> anyhow::Result<NodeEntity> {
  let data = if let Some(data) = node.data {
    Some(decrypt_data(data).await?)
  } else {
    None
  };
  Ok(NodeEntity {
    id: node.id,
    filesystem: node.filesystem,
    inode: node.inode,
    parent: node.parent,
    name: node.name,
    kind: node.kind,
    created_at: node.created_at,
    updated_at: node.updated_at,
    data,
  })
}

pub async fn decrypt_nodes(
  nodes: Vec<NodeRecord>,
) -> Vec<NodeEntity> {
  // TODO: improve error handling
  nodes
    .into_iter()
    .map(decrypt_node)
    .collect::<FuturesOrdered<_>>()
    .collect::<Vec<_>>()
    .await
    .into_iter()
    .filter_map(|node| {
      node
        .inspect_err(|e| {
          warn!("Failed to decrypt node in list | {e:#}")
        })
        .ok()
    })
    .collect()
}
