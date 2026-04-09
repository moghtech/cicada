use cicada_client::{
  api::write::{
    BatchDeleteSecrets, CreateSecret, DeleteSecret,
    RotateSecretEnvelopeKey, UpdateSecret, UpdateSecretData,
    UpdateSecretEncryptionKey,
  },
  entities::secret::SecretEntity,
};
use mogh_error::anyhow::Context as _;
use mogh_resolver::Resolve;

use crate::{
  api::write::WriteArgs,
  db::query::{self, secret::CreateSecretQuery},
  encryption::{
    decrypt_secret, decrypt_secrets, encrypt_data,
    rotate_encryption_key, rotate_envelope_key,
  },
};

impl Resolve<WriteArgs> for CreateSecret {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    let secret = query::secret::create_secret(CreateSecretQuery {
      name: self.name,
      description: self.description,
    })
    .await?;
    let secret = if let Some(data) = self.data {
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
        &secret.id.0,
      )
      .await?;
      query::secret::update_secret_data(secret.id, Some(data)).await?
    } else {
      secret
    };
    decrypt_secret(secret).await
  }
}

//

impl Resolve<WriteArgs> for UpdateSecret {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    let secret = query::secret::update_secret(self).await?;
    decrypt_secret(secret).await
  }
}

//

impl Resolve<WriteArgs> for UpdateSecretData {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    let encryption_key = if let Some(id) = self.encryption_key {
      id
    } else if let Some(id) = query::secret::get_secret(&self.id.0)
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
    let secret =
      query::secret::update_secret_data(self.id, data.into()).await?;
    decrypt_secret(secret).await
  }
}

//

impl Resolve<WriteArgs> for UpdateSecretEncryptionKey {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    let secret = query::secret::get_secret(&self.id.0).await?;
    // No-op if secret has no data.
    let Some(data) = secret.data else {
      return Ok(SecretEntity {
        id: secret.id,
        name: secret.name,
        encryption_key: None,
        data: None,
        created_at: secret.created_at,
        updated_at: secret.updated_at,
      });
    };
    // Re encrypt the envelope keys with new master key
    let data = rotate_encryption_key(
      data,
      &secret.id.0,
      self.encryption_key.0,
    )
    .await?;
    let secret =
      query::secret::update_secret_data(self.id, data.into()).await?;
    decrypt_secret(secret).await
  }
}

//

impl Resolve<WriteArgs> for RotateSecretEnvelopeKey {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    let secret = query::secret::get_secret(&self.id.0).await?;
    // No-op if secret has no data.
    let Some(data) = secret.data else {
      return Ok(SecretEntity {
        id: secret.id,
        name: secret.name,
        encryption_key: None,
        data: None,
        created_at: secret.created_at,
        updated_at: secret.updated_at,
      });
    };
    // Re encrypt data with new envelope key
    let data = rotate_envelope_key(data, &secret.id.0).await?;
    let secret =
      query::secret::update_secret_data(self.id, data.into()).await?;
    decrypt_secret(secret).await
  }
}

//

impl Resolve<WriteArgs> for DeleteSecret {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    let deleted = query::secret::delete_secret(self.id.0).await?;
    decrypt_secret(deleted).await
  }
}

//

impl Resolve<WriteArgs> for BatchDeleteSecrets {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    let deleted =
      query::secret::batch_delete_secrets(self.ids).await?;
    Ok(decrypt_secrets(deleted).await)
  }
}
