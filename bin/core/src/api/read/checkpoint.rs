use cicada_client::api::read::{GetCheckpoint, ListCheckpoints};
use mogh_resolver::Resolve;

use crate::{
  api::read::ReadArgs, db::query, encryption::decrypt_checkpoint,
  permission::ensure_client_checkpoint_target_permission,
};

impl Resolve<ReadArgs> for ListCheckpoints {
  async fn resolve(
    self,
    ReadArgs { client }: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    ensure_client_checkpoint_target_permission(client, &self.target)
      .await?;
    query::checkpoint::list_checkpoints(self.target).await
  }
}

impl Resolve<ReadArgs> for GetCheckpoint {
  async fn resolve(
    self,
    ReadArgs { client }: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    let checkpoint =
      query::checkpoint::get_checkpoint(&self.id.0).await?;
    ensure_client_checkpoint_target_permission(
      client,
      &checkpoint.target,
    )
    .await?;
    decrypt_checkpoint(checkpoint).await
  }
}
