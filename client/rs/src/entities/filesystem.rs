use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
pub struct FilesystemRecord {
  /// The unique filesystem id
  pub id: String,
  /// The name of the filesystem
  pub name: String,
}
