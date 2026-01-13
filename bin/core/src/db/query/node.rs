use anyhow::Context as _;
use axum::http::StatusCode;
use cicada_client::{
  api::{
    read::node::FindNode,
    write::node::{CreateNode, UpdateNode},
  },
  entities::{
    filesystem::FilesystemId,
    node::{NodeId, NodeKind, NodeListItem, NodeRecord},
  },
};
use futures_util::{TryStreamExt as _, stream::FuturesUnordered};
use mogh_error::AddStatusCode as _;

use crate::db::DB;

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

pub async fn get_node(
  node_id: &str,
) -> mogh_error::Result<NodeRecord> {
  DB.select::<Option<NodeRecord>>(("Node", node_id))
    .await
    .context("Failed to query database for node")?
    .context("No node found with given ID")
    .status_code(StatusCode::NOT_FOUND)
}

pub async fn find_node(
  body: FindNode,
) -> mogh_error::Result<NodeRecord> {
  DB.query(
    "
SELECT * FROM Node
WHERE filesystem = $filesystem
AND ($inode IS NONE OR inode = $inode)
AND ($parent IS NONE OR parent = $parent)
AND ($name IS NONE OR name = $name)",
  )
  .bind(("filesystem", body.filesystem))
  .bind(("inode", body.inode))
  .bind(("parent", body.parent))
  .bind(("name", body.name))
  .await
  .context("Failed to query database")?
  .take::<Option<NodeRecord>>(0)
  .context("Failed to get query result")?
  .context("Failed to find Node with given parameters.")
  .status_code(StatusCode::NOT_FOUND)
}

pub async fn create_node(
  body: CreateNode,
) -> anyhow::Result<NodeRecord> {
  DB.create("Node")
    .content(body)
    .await
    .context("Failed to create Node on database")?
    .context("Failed to create Node on database: No creation result")
}

pub async fn update_node(
  body: UpdateNode,
) -> anyhow::Result<NodeRecord> {
  DB.update(body.id.as_record_id())
    .merge(serde_json::to_value(body)?)
    .await
    .context("Failed to update Node on database")?
    .context("Failed to update Node on database: No update result")
}

pub async fn delete_node(
  id: &str,
  move_children: Option<u64>,
) -> mogh_error::Result<NodeRecord> {
  let node = get_node(id).await?;
  if matches!(node.kind, NodeKind::Folder) {
    if let Some(parent) = move_children {
      // Moves children of this node to the new parent
      DB.query("UPDATE Node SET parent = $new_parent WHERE parent = $old_parent RETURN NONE;")
        .bind(("old_parent", node.inode))
        .bind(("new_parent", parent))
        .await
        .context("Failed to move children nodes to new parent")?;
    } else {
      delete_children(node.filesystem, node.inode).await?;
    }
  }
  DB.delete(node.id.as_record_id())
    .await?
    .context("No filesystem matching given ID")
    .status_code(StatusCode::NOT_FOUND)
}

fn delete_children(
  filesystem: FilesystemId,
  parent: u64,
) -> std::pin::Pin<Box<impl Future<Output = mogh_error::Result<()>>>>
{
  Box::pin(async move {
    let children = list_nodes(Some(filesystem), Some(parent)).await?;
    // Recursively deletes any sub folders as well.
    children
      .iter()
      .map(|node| async {
        if matches!(node.kind, NodeKind::Folder) {
          delete_children(node.filesystem.clone(), node.inode)
            .await?;
        }
        mogh_error::Result::Ok(())
      })
      .collect::<FuturesUnordered<_>>()
      .try_collect::<Vec<_>>()
      .await?;
    let ids =
      children.into_iter().map(|node| node.id).collect::<Vec<_>>();
    DB.query("DELETE Node WHERE $ids.any(id) RETURN NONE;")
      .bind(("ids", ids))
      .await
      .context("Failed to delete children nodes")?;
    Ok(())
  })
}

pub async fn batch_delete_nodes(
  ids: Vec<NodeId>,
) -> anyhow::Result<()> {
  // Get the top layer nodes
  let nodes = DB
    .query("SELECT * OMIT data FROM Node WHERE $ids.any(id);")
    .bind(("ids", ids.clone()))
    .await
    .context("Failed to select nodes")?
    .take::<Vec<NodeListItem>>(0)
    .context("Invalid node query response")?;
  if nodes.is_empty() {
    return Ok(());
  }
  batch_delete_nodes_rec(nodes).await?;
  Ok(())
}

/// Queries for children of given nodes,
/// deletes the children recursively,
/// then deletes given nodes themselves.
/// 
/// This ordering ensured no dangling children are left,
/// as parents won't be deleted until after their children.
pub fn batch_delete_nodes_rec(
  nodes: Vec<NodeListItem>,
) -> std::pin::Pin<Box<impl Future<Output = anyhow::Result<()>>>> {
  Box::pin(async move {
    let filesystem_inodes = nodes
      .iter()
      .map(|node| (node.filesystem.clone(), node.inode))
      .collect::<Vec<_>>();
    let ids =
      nodes.into_iter().map(|node| node.id).collect::<Vec<_>>();
    // Get the children of top layer by querying for nodes with deleted node as parent.
    // Make sure when querying by inode to also select correct filesystem.
    let children = DB
      .query("SELECT * OMIT data FROM Node WHERE $filesystem_inodes.any([filesystem, parent]);")
      .bind(("filesystem_inodes", filesystem_inodes))
      .await
      .context("Failed to select children nodes")?
      .take::<Vec<NodeListItem>>(0)
      .context("Invalid children node query response")?;
    // Delete children layer if necessary
    if !children.is_empty() {
      batch_delete_nodes_rec(children).await?;
    }
    // Delete top layer
    DB.query("DELETE Node WHERE $ids.any(id) RETURN NONE;")
      .bind(("ids", ids))
      .await
      .context("Failed to delete nodes")?;
    Ok(())
  })
}
