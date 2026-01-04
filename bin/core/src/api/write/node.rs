use anyhow::Context as _;
use axum::http::StatusCode;
use cicada_client::{
  api::{
    read::node::{GetNode, ListNodes},
    write::node::{CreateNode, DeleteNode, UpdateNode},
  },
  entities::{
    filesystem::FilesystemId,
    node::{NodeKind, NodeRecord},
  },
};
use futures_util::{TryStreamExt, stream::FuturesUnordered};
use mogh_error::AddStatusCode as _;
use resolver_api::Resolve;

use crate::{
  api::{
    read::node::{get_node, list_nodes},
    write::WriteArgs,
  },
  db::DB,
};

#[utoipa::path(
  post,
  path = "/write/CreateNode",
  description = "Create a new node",
  request_body(content = CreateNode),
  responses(
    (status = 200, description = "The created node", body = NodeRecord),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub async fn create_node(
  body: CreateNode,
) -> mogh_error::Result<NodeRecord> {
  // let data = serde_json::to_string(&body)
  //   .context("Failed to serialize Node content")?;
  // DB.query(format!("fn::create_node({data})"))
  //   .await
  //   .context("Failed to create node on database")?
  //   .take::<Option<_>>(0)
  //   .context("Failed to create node on database")?
  //   .context("No creation result")
  //   .map_err(Into::into)
  DB.create("Node")
    .content(body)
    .await
    .context("Failed to create Node on database")?
    .context("Failed to create Node on database: No creation result")
    .map_err(Into::into)
}

impl Resolve<WriteArgs> for CreateNode {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    create_node(self).await
  }
}

//

#[utoipa::path(
  post,
  path = "/write/UpdateNode",
  description = "Update a node",
  request_body(content = UpdateNode),
  responses(
    (status = 200, description = "The updated node", body = NodeRecord),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub async fn update_node(
  body: UpdateNode,
) -> mogh_error::Result<NodeRecord> {
  // let update = serde_json::to_string(&body)
  //   .context("Failed to serialize MERGE update")?;
  // DB.query(format!(r#"UPDATE $id MERGE {update}"#))
  //   .bind(("id", body.id))
  //   .await
  //   .context("Failed to update Node on database")?
  //   .take::<Option<_>>(0)?
  //   .context("Failed to update Node on database: No update result")
  //   .map_err(Into::into)
  DB.update(body.id.as_record_id())
    .merge(serde_json::to_value(body)?)
    .await
    .context("Failed to update Node on database")?
    .context("Failed to update Node on database: No update result")
    .map_err(Into::into)
}

impl Resolve<WriteArgs> for UpdateNode {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    update_node(self).await
  }
}

//

#[utoipa::path(
  post,
  path = "/write/DeleteNode",
  description = "Delete a node",
  request_body(content = DeleteNode),
  responses(
    (status = 200, description = "The deleted node", body = NodeRecord),
    (status = 404, description = "Node not found", body = mogh_error::Serror),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub async fn delete_node(
  body: DeleteNode,
) -> mogh_error::Result<NodeRecord> {
  let node = get_node(GetNode { id: body.id }).await?;
  if matches!(node.kind, NodeKind::Folder) {
    if let Some(parent) = body.move_children {
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

impl Resolve<WriteArgs> for DeleteNode {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    delete_node(self).await
  }
}

fn delete_children(
  filesystem: FilesystemId,
  parent: u64,
) -> std::pin::Pin<Box<impl Future<Output = mogh_error::Result<()>>>>
{
  Box::pin(async move {
    let children = list_nodes(ListNodes {
      filesystem: Some(filesystem),
      parent: Some(parent),
    })
    .await?;
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
