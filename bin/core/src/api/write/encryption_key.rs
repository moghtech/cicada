use axum::http::StatusCode;
use cicada_client::{
  api::write::{
    CreateEncryptionKey, InitializeEncryptionKey,
    InitializeEncryptionKeyResponse, UninitializeEncryptionKey,
    UninitializeEncryptionKeyResponse, UpdateEncryptionKey,
  },
  entities::encryption_key::EncryptionKeyKind,
};
use mogh_encryption::{BASE64URL, xchacha20poly1305};
use mogh_error::AddStatusCodeError;
use mogh_error::anyhow::{Context as _, anyhow};
use mogh_resolver::Resolve;

use crate::{
  api::write::WriteArgs,
  db::query::{self, encryption_key::CreateEncryptionKeyQuery},
  encryption::{base64url_to_array, encryption_keys},
};

impl Resolve<WriteArgs> for CreateEncryptionKey {
  async fn resolve(
    mut self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    client.admin_only()?;

    // Parse or generate key
    let (key, encoded_key) = if let Some(key) =
      self.key.map(|k| k.trim().to_string())
      && !key.is_empty()
    {
      (base64url_to_array::<32>(key.as_bytes())?, key)
    } else {
      let key = rand::random::<[u8; 32]>();
      (key, BASE64URL.encode(&key))
    };
    // Fix the stored data
    match self.kind {
      EncryptionKeyKind::Memory => self.key = None,
      EncryptionKeyKind::Disk => self.key = Some(encoded_key.clone()),
    }
    let verification: [u8; 32] = rand::random();
    let mogh_encryption::EncryptedData {
      data: verification_encrypted,
      nonce: verification_nonce,
    } = xchacha20poly1305::EncryptionProvider::default().encrypt(
      &verification,
      key,
      &(),
    )?;
    let mut encryption_key =
      query::encryption_key::create_encryption_key(
        CreateEncryptionKeyQuery {
          name: self.name,
          kind: self.kind,
          key: self.key,
          verification: BASE64URL.encode(&verification),
          verification_encrypted,
          verification_nonce,
        },
      )
      .await?;
    // Insert the key into the in memory map for immediate usage.
    encryption_keys().insert(encryption_key.id.0.clone(), key);
    // This response always includes the encoded key,
    // if in memory key then this is only time it is made available to user.
    encryption_key.key = Some(encoded_key);
    Ok(encryption_key)
  }
}

//

impl Resolve<WriteArgs> for UpdateEncryptionKey {
  async fn resolve(
    self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    client.admin_only()?;
    query::encryption_key::update_encryption_key(self).await
  }
}

//

impl Resolve<WriteArgs> for InitializeEncryptionKey {
  async fn resolve(
    self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    client.admin_only()?;

    let encryption_key =
      query::encryption_key::get_encryption_key(&self.id.0).await?;

    // Validate it is Memory kind key
    if !matches!(encryption_key.kind, EncryptionKeyKind::Memory) {
      return Err(
        anyhow!("Key at id is not 'Memory' kind")
          .status_code(StatusCode::BAD_REQUEST),
      );
    }

    // Get the decoded master key
    let key = base64url_to_array::<32>(self.key.as_bytes())?;

    // Test verification decrypt
    let verification = xchacha20poly1305::decrypt(
      &mogh_encryption::EncryptedData {
        data: encryption_key.verification_encrypted,
        nonce: encryption_key.verification_nonce,
      },
      key,
      &(),
    )
    .context("Incoming encryption key failed verification")?;

    if BASE64URL.encode(&verification) != encryption_key.verification
    {
      return Err(
        anyhow!("Incoming encryption key failed verification")
          .status_code(StatusCode::BAD_REQUEST),
      );
    }

    // After verified, insert the key for active use.
    encryption_keys().insert(encryption_key.id.0, key);

    Ok(InitializeEncryptionKeyResponse {})
  }
}

//

impl Resolve<WriteArgs> for UninitializeEncryptionKey {
  async fn resolve(
    self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    client.admin_only()?;
    Ok(UninitializeEncryptionKeyResponse {
      removed: encryption_keys().remove(&self.id.0),
    })
  }
}
