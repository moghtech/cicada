use cicada_client::api::write::{
  BatchDeleteCheckpoints, DeleteCheckpoint,
  RotateCheckpointEnvelopeKey, UpdateCheckpoint,
  UpdateCheckpointEncryptionKey,
};
use mogh_resolver::Resolve;

use crate::{
  api::write::WriteArgs,
  db::query,
  encryption::{
    decrypt_checkpoint, decrypt_checkpoints, rotate_encryption_key,
    rotate_envelope_key,
  },
};

//

impl Resolve<WriteArgs> for UpdateCheckpoint {
  async fn resolve(
    self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    client.admin_only()?;
    let checkpoint =
      query::checkpoint::update_checkpoint(self).await?;
    decrypt_checkpoint(checkpoint).await
  }
}

//

impl Resolve<WriteArgs> for UpdateCheckpointEncryptionKey {
  async fn resolve(
    self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    client.admin_only()?;
    let checkpoint =
      query::checkpoint::get_checkpoint(&self.id.0).await?;
    // Re encrypt the envelope keys with new master key
    let data = rotate_encryption_key(
      &checkpoint.encryption_key.0,
      checkpoint.data,
      checkpoint.target.associated_data(),
      &self.encryption_key.0,
    )
    .await?;
    let checkpoint = query::checkpoint::update_checkpoint_data(
      self.id,
      self.encryption_key,
      data,
    )
    .await?;
    decrypt_checkpoint(checkpoint).await
  }
}

//

impl Resolve<WriteArgs> for RotateCheckpointEnvelopeKey {
  async fn resolve(
    self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    client.admin_only()?;
    let checkpoint =
      query::checkpoint::get_checkpoint(&self.id.0).await?;
    // Re encrypt data with new envelope key
    let data = rotate_envelope_key(
      &checkpoint.encryption_key.0,
      checkpoint.data,
      checkpoint.target.associated_data(),
    )
    .await?;
    let checkpoint = query::checkpoint::update_checkpoint_data(
      self.id,
      checkpoint.encryption_key,
      data,
    )
    .await?;
    decrypt_checkpoint(checkpoint).await
  }
}

//

impl Resolve<WriteArgs> for DeleteCheckpoint {
  async fn resolve(
    self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    client.admin_only()?;
    let deleted =
      query::checkpoint::delete_checkpoint(self.id.0).await?;
    decrypt_checkpoint(deleted).await
  }
}

//

impl Resolve<WriteArgs> for BatchDeleteCheckpoints {
  async fn resolve(
    self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    client.admin_only()?;
    let deleted =
      query::checkpoint::batch_delete_checkpoints(self.ids).await?;
    Ok(decrypt_checkpoints(deleted).await)
  }
}
