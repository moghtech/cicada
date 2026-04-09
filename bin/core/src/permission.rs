use cicada_client::entities::{
  filesystem::FilesystemId, node::NodeId,
};
use mogh_error::anyhow;

pub async fn check_node_permission(
  filesystem: FilesystemId,
  node: NodeId,
  write_required: bool,
  client: &crate::auth::middleware::Client,
) -> anyhow::Result<()> {
  todo!()
}
