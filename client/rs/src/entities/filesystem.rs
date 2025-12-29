use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use surrealdb_types::{RecordIdKey, SurrealValue};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
pub struct FilesystemRecord {
  /// The unique filesystem id
  pub id: FilesystemId,
  /// The name of the filesystem
  pub name: String,
}

#[typeshare(serialized_as = "string")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesystemId(pub String);

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
