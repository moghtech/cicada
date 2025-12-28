use typeshare::typeshare;

/// Configuration for Cicada
pub mod config;
/// Represents virtual filesystems which can be mounted to clients.
pub mod filesystem;
/// Subtypes of [LogConfig][logger::LogConfig].
pub mod logger;
/// Nodes represent entries in a filesystem.
/// They represent either Files or Folders.
pub mod node;

#[typeshare(serialized_as = "number")]
pub type U64 = u64;
