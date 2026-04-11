use cicada_client::api::read::{GetCheckpoint, ListCheckpoints};
use mogh_resolver::Resolve;

use crate::{
  api::read::ReadArgs, db::query, encryption::decrypt_checkpoint,
  permission::ensure_client_filesystem_permission,
};

impl Resolve<ReadArgs> for ListCheckpoints {
  async fn resolve(
    self,
    ReadArgs { client }: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    let node = query::node::get_node(&self.node.0).await?;
    // Don't know filesystem id for perm check until node query.
    ensure_client_filesystem_permission(
      client,
      node.filesystem.clone(),
      false,
    )
    .await?;
    query::checkpoint::list_checkpoints(self.node).await
  }
}

impl Resolve<ReadArgs> for GetCheckpoint {
  async fn resolve(
    self,
    ReadArgs { client }: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    let checkpoint =
      query::checkpoint::get_checkpoint(&self.id.0).await?;
    let node = query::node::get_node(&checkpoint.node.0).await?;
    // Don't know filesystem id for perm check until node query.
    ensure_client_filesystem_permission(
      client,
      node.filesystem.clone(),
      false,
    )
    .await?;
    decrypt_checkpoint(checkpoint).await
  }
}
