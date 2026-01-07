use anyhow::Context as _;
use cicada_client::entities::{
  filesystem::FilesystemId,
  node::{NodeListItem, NodeRecord},
};

use crate::db::DB;

pub async fn get_node(node_id: &str) -> anyhow::Result<NodeRecord> {
  DB.select::<Option<NodeRecord>>(("Node", node_id))
    .await
    .context("Failed to query database for node")?
    .context("No node found with given ID")
}

pub async fn list_nodes(
  filesystem: Option<FilesystemId>,
  parent: Option<u64>,
) -> anyhow::Result<Vec<NodeListItem>> {
  DB.query(
    "
SELECT * OMIT data FROM Node 
WHERE ($filesystem IS NONE OR filesystem = $filesystem)
AND ($parent IS NONE OR parent = $parent)",
  )
  .bind(("filesystem", filesystem))
  .bind(("parent", parent))
  .await
  .context("Failed to query database for nodes")?
  .take(0)
  .context("Failed to get node query result")
}
