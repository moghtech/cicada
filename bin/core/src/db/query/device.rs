use axum::http::StatusCode;
use cicada_client::{
  api::write::{CreateDevice, UpdateDevice},
  entities::device::{DeviceId, DeviceRecord},
};
use mogh_error::AddStatusCode as _;
use mogh_error::anyhow::Context as _;

use crate::db::DB;

pub async fn list_all_devices()
-> mogh_error::Result<Vec<DeviceRecord>> {
  DB.select("Device")
    .await
    .context("Failed to query for Devices")
    .map_err(Into::into)
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
) -> mogh_error::Result<Option<DeviceRecord>> {
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
) -> mogh_error::Result<DeviceRecord> {
  DB.create("Device")
    .content(body)
    .await
    .context("Failed to create Device on database")?
    .context(
      "Failed to create Device on database: No creation result",
    )
    .map_err(Into::into)
}

pub async fn update_device(
  body: UpdateDevice,
) -> mogh_error::Result<DeviceRecord> {
  DB.query("UPDATE $id MERGE fn::object_strip_none($body);")
    .bind(("id", body.id.clone()))
    .bind(("body", body))
    .await
    .context("Failed to query database")?
    .take::<Option<DeviceRecord>>(0)
    .context("Failed to get query result")?
    .context("Failed to find device with given parameters.")
    .status_code(StatusCode::NOT_FOUND)
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
) -> mogh_error::Result<Vec<DeviceRecord>> {
  DB.query("DELETE Device WHERE id IN $ids RETURN BEFORE;")
    .bind(("ids", ids))
    .await
    .context("Failed to delete devices")?
    .take(0)
    .context("Invalid delete device query response")
    .map_err(Into::into)
}
