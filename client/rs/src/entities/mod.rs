use typeshare::typeshare;

/// Nodes represent entries in the filesystem.
/// They represent either Files or Folders.
///
pub mod node;

#[typeshare(serialized_as = "number")]
pub type U64 = u64;
