use axum::http::StatusCode;
use cicada_client::{
  api::write::UpdateCheckpoint,
  entities::{
    EncryptedData,
    checkpoint::{
      CheckpointId, CheckpointListItem, CheckpointRecord,
    },
    node::NodeId,
  },
};
use mogh_error::AddStatusCode as _;
use mogh_error::anyhow::Context as _;
use surrealdb_types::SurrealValue;

use crate::db::DB;

pub async fn list_checkpoints(
  node: NodeId,
) -> mogh_error::Result<Vec<CheckpointListItem>> {
  DB.query(
    "
SELECT *, data.encryption_key AS encryption_key OMIT data FROM Checkpoint
WHERE node = $node
ORDER BY created_at DESC;",
  )
  .bind(("node", node))
  .await
  .context("Failed to query database for checkpoints")?
  .take(0)
  .context("Failed to get checkpoint query result")
  .map_err(Into::into)
}

pub async fn get_checkpoint(
  checkpoint_id: &str,
) -> mogh_error::Result<CheckpointRecord> {
  DB.select::<Option<CheckpointRecord>>(("Checkpoint", checkpoint_id))
    .await
    .context("Failed to query database for checkpoint")?
    .context("No checkpoint found with given ID")
    .status_code(StatusCode::NOT_FOUND)
}

// pub async fn create_checkpoint(
//   body: CreateCheckpoint,
// ) -> mogh_error::Result<CheckpointRecord> {
//   DB.create("Checkpoint")
//     .content(body)
//     .await
//     .context("Failed to create Checkpoint on database")?
//     .context(
//       "Failed to create Checkpoint on database: No creation result",
//     )
//     .map_err(Into::into)
// }

pub async fn update_checkpoint(
  body: UpdateCheckpoint,
) -> mogh_error::Result<CheckpointRecord> {
  DB.query("UPDATE $id MERGE fn::object_strip_none($body);")
    .bind(("id", body.id.clone()))
    .bind(("body", body))
    .await
    .context("Failed to query database")?
    .take::<Option<CheckpointRecord>>(0)
    .context("Failed to get query result")?
    .context("Failed to find checkpoint with given parameters.")
    .status_code(StatusCode::NOT_FOUND)
}

pub async fn update_checkpoint_data(
  id: CheckpointId,
  data: Option<EncryptedData>,
) -> mogh_error::Result<CheckpointRecord> {
  #[derive(SurrealValue)]
  struct UpdateCheckpointDataQuery {
    data: Option<EncryptedData>,
  }
  DB.update(id.as_record_id())
    .merge(UpdateCheckpointDataQuery { data })
    .await
    .context("Failed to update Checkpoint on database")?
    .context(
      "Failed to update Checkpoint on database: No update result",
    )
    .map_err(Into::into)
}

pub async fn delete_checkpoint(
  id: String,
) -> mogh_error::Result<CheckpointRecord> {
  DB.delete(("Checkpoint", id))
    .await?
    .context("No Checkpoint matching given ID")
    .status_code(StatusCode::NOT_FOUND)
}

pub async fn batch_delete_checkpoints(
  ids: Vec<CheckpointId>,
) -> mogh_error::Result<Vec<CheckpointRecord>> {
  DB.query("DELETE Checkpoint WHERE id IN $ids RETURN BEFORE;")
    .bind(("ids", ids))
    .await
    .context("Failed to delete checkpoints")?
    .take(0)
    .context("Invalid delete checkpoints query response")
    .map_err(Into::into)
}
