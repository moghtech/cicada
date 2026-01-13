use anyhow::Context as _;
use axum::http::StatusCode;
use cicada_client::{
  api::write::device::{CreateDevice, UpdateDevice},
  entities::device::{DeviceId, DeviceRecord},
};
use mogh_error::AddStatusCode as _;

use crate::db::DB;

pub async fn list_all_devices() -> anyhow::Result<Vec<DeviceRecord>> {
  DB.select("Device")
    .await
    .context("Failed to query for Devices")
}

pub async fn get_device(
  device_id: &str,
) -> mogh_error::Result<DeviceRecord> {
  DB.select::<Option<DeviceRecord>>(("Device", device_id))
    .await
    .context("Failed to query database for device")?
    .context("No device found with given ID")
    .status_code(StatusCode::NOT_FOUND)
}

pub async fn find_device_with_public_key(
  public_key: String,
) -> anyhow::Result<Option<DeviceRecord>> {
  let device = DB
    .query("SELECT * FROM Device WHERE public_key = $public_key")
    .bind(("public_key", public_key))
    .await
    .context("Failed to query database for device")?
    .take::<Vec<DeviceRecord>>(0)
    .context("Failed to deserialize DeviceRecord")?
    .pop();
  Ok(device)
}

pub async fn create_device(
  body: CreateDevice,
) -> anyhow::Result<DeviceRecord> {
  DB.create("Device")
    .content(body)
    .await
    .context("Failed to create Device on database")?
    .context(
      "Failed to create Device on database: No creation result",
    )
}

pub async fn update_device(
  body: UpdateDevice,
) -> anyhow::Result<DeviceRecord> {
  DB.update(body.id.as_record_id())
    .merge(serde_json::to_value(body)?)
    .await
    .context("Failed to update Device on database")?
    .context("Failed to update Device on database: No update result")
}

pub async fn delete_device(
  id: String,
) -> mogh_error::Result<DeviceRecord> {
  DB.delete(("Device", id))
    .await?
    .context("No Device matching given ID")
    .status_code(StatusCode::NOT_FOUND)
}

pub async fn batch_delete_devices(
  ids: Vec<DeviceId>,
) -> anyhow::Result<Vec<DeviceRecord>> {
  DB.query("DELETE Device WHERE $ids.any(id);")
    .bind(("ids", ids))
    .await
    .context("Failed to delete devices")?
    .take(0)
    .context("Invalid delete device query response")
}
