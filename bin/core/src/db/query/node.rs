use axum::http::StatusCode;
use cicada_client::{
  api::{read::node::FindNode, write::node::UpdateNode},
  entities::{
    EncryptedData,
    filesystem::FilesystemId,
    node::{NodeId, NodeKind, NodeListItem, NodeRecord},
  },
};
use mogh_error::AddStatusCode as _;
use mogh_error::anyhow::Context as _;
use surrealdb_types::SurrealValue;

use crate::db::DB;

pub async fn list_nodes(
  filesystem: Option<FilesystemId>,
  parent: Option<u64>,
) -> mogh_error::Result<Vec<NodeListItem>> {
  DB.query(
    "
SELECT * OMIT data FROM Node 
WHERE ($filesystem IS NONE OR filesystem = $filesystem)
AND ($parent IS NONE OR parent = $parent)
ORDER BY kind DESC, name COLLATE ASC;",
  )
  .bind(("filesystem", filesystem))
  .bind(("parent", parent))
  .await
  .context("Failed to query database for nodes")?
  .take(0)
  .context("Failed to get node query result")
  .map_err(Into::into)
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

pub async fn get_node_list_item(
  node_id: String,
) -> mogh_error::Result<NodeListItem> {
  DB.query(r#"SELECT * OMIT data FROM type::record("Node", $id)"#)
    .bind(("id", node_id))
    .await
    .context("Failed to query database for node")?
    .take::<Option<NodeListItem>>(0)
    .context("Invalid get node list item query response")?
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

#[derive(SurrealValue)]
pub struct CreateNodeQuery {
  pub filesystem: Option<FilesystemId>,
  pub parent: Option<u64>,
  pub name: String,
  pub kind: Option<NodeKind>,
  /// The file permission integer.
  /// Usually represented as octet like 0o644.
  /// If not provided, will use defaults:
  /// - Folder: 0o755
  /// - File: 0o644
  pub perm: Option<u16>,
}

pub async fn create_node(
  body: CreateNodeQuery,
) -> mogh_error::Result<NodeRecord> {
  DB.create("Node")
    .content(body)
    .await
    .context("Failed to create Node on database")?
    .context("Failed to create Node on database: No creation result")
    .map_err(Into::into)
}

pub async fn update_node(
  body: UpdateNode,
) -> mogh_error::Result<NodeRecord> {
  DB.update(body.id.as_record_id())
    .merge(serde_json::to_value(body)?)
    .await
    .context("Failed to update Node on database")?
    .context("Failed to update Node on database: No update result")
    .status_code(StatusCode::NOT_FOUND)
}

pub async fn update_node_data(
  id: NodeId,
  data: Option<EncryptedData>,
) -> mogh_error::Result<NodeRecord> {
  #[derive(SurrealValue)]
  struct UpdateNodeDataQuery {
    data: Option<EncryptedData>,
  }
  DB.update(id.as_record_id())
    .merge(UpdateNodeDataQuery { data })
    .await
    .context("Failed to update Node on database")?
    .context("Failed to update Node on database: No update result")
    .map_err(Into::into)
}

pub async fn delete_node(
  id: String,
  move_children: Option<u64>,
) -> mogh_error::Result<Vec<NodeRecord>> {
  let node = get_node_list_item(id).await?;
  if matches!(node.kind, NodeKind::Folder) {
    if let Some(parent) = move_children {
      // Moves children of this node to the new parent
      DB.query("UPDATE Node SET parent = $new_parent WHERE parent = $old_parent RETURN NONE;")
        .bind(("old_parent", node.inode))
        .bind(("new_parent", parent))
        .await
        .context("Failed to move children nodes to new parent")?;
    } else {
      // Do standard recursive batch delete
      let mut deleted = Vec::new();
      batch_delete_nodes_rec(vec![node], &mut deleted).await?;
      return Ok(deleted);
    }
  }
  // Node is either file, or move_children is passed.
  // In either case, only one deleted node.
  let deleted = DB
    .delete(node.id.as_record_id())
    .await?
    .context("No node matching given ID")
    .status_code(StatusCode::NOT_FOUND)?;
  Ok(vec![deleted])
}

pub async fn batch_delete_nodes(
  ids: Vec<NodeId>,
) -> mogh_error::Result<Vec<NodeRecord>> {
  if ids.is_empty() {
    return Ok(Vec::new());
  }

  // Start recursive delete from top layer of nodes,
  // collecting any deleted records.
  let mut deleted = Vec::new();
  let nodes = DB
    .query("SELECT * OMIT data FROM Node WHERE $ids.any(id);")
    .bind(("ids", ids.clone()))
    .await
    .context("Failed to select nodes")?
    .take::<Vec<NodeListItem>>(0)
    .context("Invalid node query response")?;
  batch_delete_nodes_rec(nodes, &mut deleted).await?;
  Ok(deleted)
}

/// Queries for children of given nodes,
/// deletes the children recursively,
/// then deletes given nodes themselves.
///
/// This ordering ensured no dangling children are left,
/// as parents won't be deleted until after their children.
pub fn batch_delete_nodes_rec(
  nodes: Vec<NodeListItem>,
  deleted: &mut Vec<NodeRecord>,
) -> std::pin::Pin<Box<impl Future<Output = mogh_error::Result<()>>>>
{
  Box::pin(async {
    if nodes.is_empty() {
      return Ok(());
    }

    let folder_inodes = nodes
      .iter()
      .filter(|node| matches!(node.kind, NodeKind::Folder))
      .map(|node| (node.filesystem.clone(), node.inode))
      .collect::<Vec<_>>();

    // If theres any folder included, check to delete children
    if !folder_inodes.is_empty() {
      // Get the children of top layer by querying for nodes with deleted node as parent.
      let children = DB
      .query("SELECT * OMIT data FROM Node WHERE $folder_inodes.any([filesystem, parent]);")
      .bind(("folder_inodes", folder_inodes))
      .await
      .context("Failed to select children nodes")?
      .take::<Vec<NodeListItem>>(0)
      .context("Invalid children node query response")?;
      // Delete children layer if necessary
      if !children.is_empty() {
        batch_delete_nodes_rec(children, deleted).await?;
      }
    }

    // Delete top layer
    let ids =
      nodes.into_iter().map(|node| node.id).collect::<Vec<_>>();
    let more = DB
      .query("DELETE Node WHERE $ids.any(id) RETURN BEFORE;")
      .bind(("ids", ids))
      .await
      .context("Failed to delete nodes")?
      .take::<Vec<NodeRecord>>(0)
      .context("Invalid delete node query response")?;
    deleted.extend(more);
    Ok(())
  })
}
