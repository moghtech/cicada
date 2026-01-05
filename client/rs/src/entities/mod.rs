use typeshare::typeshare;

/// Configuration for Cicada
pub mod config;
/// Represents virtual filesystems which can be mounted to clients.
pub mod filesystem;
/// Nodes represent entries in a filesystem.
/// They represent either Files or Folders.
pub mod node;
///
pub mod user;

#[typeshare(serialized_as = "number")]
pub type U64 = u64;
#[typeshare(serialized_as = "string")]
pub type Iso8601Timestamp = surrealdb_types::Datetime;
