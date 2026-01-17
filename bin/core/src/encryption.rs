use std::sync::OnceLock;

use anyhow::{Context, anyhow};
use cicada_client::entities::{
  EncryptedData,
  encryption_key::{EncryptionKeyId, EncryptionKeyKind},
  node::{NodeEntity, NodeRecord},
};
use dashmap::DashMap;
use futures_util::{StreamExt as _, stream::FuturesOrdered};
use mogh_encryption::{
  AssociatedData, BASE64URL, EnvelopeEncryptedData, xchacha20poly1305,
};

use crate::db::query;

#[derive(Default)]
pub struct EncryptionKeys(DashMap<String, Option<[u8; 32]>>);

impl EncryptionKeys {
  pub async fn get_or_insert_key(
    &self,
    id: &str,
  ) -> anyhow::Result<[u8; 32]> {
    if let Some(master_key) = self.0.get(id) {
      // Early exit without db query if already known
      // such query wouldn't be able to get the key.
      let value =
        master_key.value().context("Missing encryption key at id")?;
      return Ok(value);
    }
    let encryption_key =
      query::encryption_key::get_encryption_key(id)
        .await
        .map_err(|e| e.error)?;
    match encryption_key.kind {
      EncryptionKeyKind::Memory => {
        // Insert None on the map if key is known to be in memory.
        // It needs to be initialized via API call after every app startup.
        self.0.insert(id.to_string(), None);
        Err(anyhow!(
          "Missing in memory encryption key. Initialize it via API call."
        ))
      }
      EncryptionKeyKind::Disk => {
        let key = encryption_key
          .key
          .context("On disk encryption key missing stored key")?;
        let key = base64url_to_array(key.as_bytes())?;
        self.0.insert(id.to_string(), Some(key));
        Ok(key)
      }
    }
  }

  pub fn contains_key(&self, encryption_key_id: &str) -> bool {
    // The key must exist in map and be non null
    self
      .0
      .get(encryption_key_id)
      .map(|e| e.value().is_some())
      .unwrap_or_default()
  }

  pub fn insert_key(&self, id: String, key: [u8; 32]) {
    self.0.insert(id, Some(key));
  }
}

pub fn encryption_keys() -> &'static EncryptionKeys {
  static ENCRYPTION_KEYS: OnceLock<EncryptionKeys> = OnceLock::new();
  ENCRYPTION_KEYS.get_or_init(Default::default)
}

pub async fn encrypt_data<A: AssociatedData>(
  encryption_key_id: String,
  data: &[u8],
  associated_data: &A,
) -> anyhow::Result<EncryptedData> {
  let master_key = encryption_keys()
    .get_or_insert_key(&encryption_key_id)
    .await?;
  let EnvelopeEncryptedData { key, data } =
    xchacha20poly1305::EncryptionProvider::default()
      .envelope_encrypt(data, master_key, associated_data)?;
  Ok(EncryptedData {
    encryption_key: EncryptionKeyId(encryption_key_id),
    key: key.data,
    key_nonce: key.nonce,
    data: data.data,
    data_nonce: data.nonce,
  })
}

/// If Err, decryption failed.
/// If None, missing encryption key.
pub async fn decrypt_data<A: AssociatedData, T: TryFrom<Vec<u8>>>(
  data: EncryptedData,
  associated_data: &A,
) -> anyhow::Result<Option<T>>
where
  T::Error: Send + Sync + std::error::Error + 'static,
{
  let Ok(master_key) = encryption_keys()
    .get_or_insert_key(&data.encryption_key.0)
    .await
  else {
    return Ok(None);
  };
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
    associated_data,
  )?;
  let data = data
    .try_into()
    .context("Failed to convert data from bytes")?;
  Ok(Some(data))
}

/// Re-encrypts the envelope keys using new master. Does not re-encrypt the data itself, so its cheap.
pub async fn rotate_encryption_key<A: AssociatedData>(
  data: EncryptedData,
  associated_data: &A,
  new_encryption_key_id: String,
) -> anyhow::Result<EncryptedData> {
  let old_master_key = encryption_keys()
    .get_or_insert_key(&data.encryption_key.0)
    .await?;
  let new_master_key = encryption_keys()
    .get_or_insert_key(&new_encryption_key_id)
    .await?;
  // Decrypt just the envelope keys using old master
  let key = xchacha20poly1305::decrypt(
    &mogh_encryption::EncryptedData {
      data: data.key,
      nonce: data.key_nonce,
    },
    old_master_key,
    associated_data,
  )?;
  let mogh_encryption::EncryptedData {
    data: key,
    nonce: key_nonce,
  } = xchacha20poly1305::EncryptionProvider::default().encrypt(
    &key,
    new_master_key,
    associated_data,
  )?;
  Ok(EncryptedData {
    encryption_key: EncryptionKeyId(new_encryption_key_id),
    key,
    key_nonce,
    data: data.data,
    data_nonce: data.data_nonce,
  })
}

/// Decrypts data, regenerates envelope key, re-encrypts
pub async fn rotate_envelope_key<A: AssociatedData>(
  data: EncryptedData,
  associated_data: &A,
) -> anyhow::Result<EncryptedData> {
  let encryption_key = data.encryption_key.clone();
  let data = decrypt_data::<A, Vec<u8>>(data, associated_data)
    .await?
    .context("Cannot rotate envelope key without master key")?;
  encrypt_data(encryption_key.0, &data, associated_data).await
}

pub async fn decrypt_node(
  node: NodeRecord,
) -> anyhow::Result<NodeEntity> {
  let (data, missing_key) = if let Some(data) = node.data {
    let key = data.encryption_key.clone();
    if let Some(data) = decrypt_data(data, &node.id.0).await? {
      (Some(data), None)
    } else {
      (None, Some(key))
    }
  } else {
    (None, None)
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
    missing_key,
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

pub fn base64url_to_array<const LENGTH: usize>(
  base64url: &[u8],
) -> anyhow::Result<[u8; LENGTH]> {
  let vec = BASE64URL
    .decode(base64url)
    .context("Invalid base64url encoding")?;
  vec
    .try_into()
    .map_err(|_| anyhow!("Invalid decoded base64url bytes length"))
}
