use axum::http::StatusCode;
use cicada_client::entities::{
  filesystem::FilesystemId,
  node::NodeId,
  policy::PolicyRecord,
};
use mogh_error::{
  AddStatusCode as _, AddStatusCodeError as _,
  anyhow::Context as _,
};

use surrealdb_types::SurrealValue;

use crate::{auth::middleware::Client, db::DB};

#[derive(SurrealValue)]
struct ParentInfo {
  #[allow(dead_code)]
  inode: u64,
  parent: u64,
}

#[derive(SurrealValue)]
struct IdOnly {
  id: NodeId,
}

pub async fn check_node_permission(
  filesystem: FilesystemId,
  node: NodeId,
  write_required: bool,
  client: &Client,
) -> mogh_error::Result<()> {
  let groups = match client {
    Client::User(user) => {
      if user.admin {
        return Ok(());
      }
      &user.groups
    }
    Client::Device(device) => &device.groups,
    Client::OnboardingKey(_) => {
      return Err(
        mogh_error::anyhow::anyhow!(
          "OnboardingKey clients cannot access nodes"
        )
        .status_code(StatusCode::FORBIDDEN),
      );
    }
  };

  let policies: Vec<PolicyRecord> = DB
    .query(
      "SELECT * FROM Policy WHERE groups CONTAINSANY $groups",
    )
    .bind(("groups", groups.clone()))
    .await
    .context("Failed to query policies")?
    .take(0)
    .context("Failed to get policy query result")?;

  // Check if any policy grants access to the whole filesystem
  if policies.iter().any(|policy| {
    let fs_access =
      policy.filesystems.iter().any(|fs| fs.0 == filesystem.0);
    if write_required {
      fs_access && policy.write
    } else {
      fs_access
    }
  }) {
    return Ok(());
  }

  // Collect all policy node IDs for quick lookup
  let policy_node_ids: Vec<&str> = policies
    .iter()
    .filter(|p| !write_required || p.write)
    .flat_map(|p| p.nodes.iter().map(|n| n.0.as_str()))
    .collect();

  if policy_node_ids.is_empty() {
    return Err(
      mogh_error::anyhow::anyhow!("Permission denied")
        .status_code(StatusCode::FORBIDDEN),
    );
  }

  // Walk up the node tree checking if the node or any
  // ancestor is referenced by a matching policy.
  let mut current_id = node;
  loop {
    if policy_node_ids.contains(&current_id.0.as_str()) {
      return Ok(());
    }

    // Get the current node's parent inode
    let node_record = DB
      .query(
        "SELECT inode, parent FROM ONLY $node_id",
      )
      .bind(("node_id", current_id))
      .await
      .context("Failed to query node")?
      .take::<Option<ParentInfo>>(0)
      .context("Failed to get node query result")?
      .context("Node not found")
      .status_code(StatusCode::NOT_FOUND)?;

    // Root node has parent == 0
    if node_record.parent == 0 {
      break;
    }

    // Find the parent node by inode in the same filesystem
    let parent = DB
      .query(
        "SELECT id FROM ONLY Node
         WHERE filesystem = $filesystem
         AND inode = $parent",
      )
      .bind(("filesystem", filesystem.clone()))
      .bind(("parent", node_record.parent))
      .await
      .context("Failed to query parent node")?
      .take::<Option<IdOnly>>(0)
      .context("Failed to get parent node query result")?;

    match parent {
      Some(parent) => current_id = parent.id,
      None => break,
    }
  }

  Err(
    mogh_error::anyhow::anyhow!("Permission denied")
      .status_code(StatusCode::FORBIDDEN),
  )
}
