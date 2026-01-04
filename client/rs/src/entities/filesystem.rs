use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use surrealdb_types::{RecordId, RecordIdKey, SurrealValue};
use typeshare::typeshare;

use crate::entities::Iso8601Timestamp;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct FilesystemRecord {
  /// The unique filesystem id
  pub id: FilesystemId,
  /// The name of the filesystem
  pub name: String,
  /// Created at as ISO8601 timestamp.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub created_at: Iso8601Timestamp,
  /// Updated at as ISO8601 timestamp.
  #[cfg_attr(feature = "utoipa", schema(value_type = String))]
  pub updated_at: Iso8601Timestamp,
}

#[typeshare(serialized_as = "string")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct FilesystemId(pub String);

impl FilesystemId {
  pub fn as_record_id(&self) -> RecordId {
    RecordId::new("Filesystem", self.0.as_str())
  }
}

impl SurrealValue for FilesystemId {
  fn kind_of() -> surrealdb_types::Kind {
    surrealdb_types::Kind::Record(vec![])
  }

  fn into_value(self) -> surrealdb_types::Value {
    surrealdb_types::Value::RecordId(surrealdb_types::RecordId::new(
      "Filesystem",
      self.0,
    ))
  }

  fn from_value(value: surrealdb_types::Value) -> anyhow::Result<Self>
  where
    Self: Sized,
  {
    let surrealdb_types::Value::RecordId(id) = value else {
      return Err(anyhow!("Value is not RecordId"));
    };
    let RecordIdKey::String(id) = id.key else {
      return Err(anyhow!("RecordIdKey is not String"));
    };
    Ok(Self(id))
  }
}
