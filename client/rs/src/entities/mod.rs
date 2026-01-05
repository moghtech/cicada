use typeshare::typeshare;

/// Configuration for Cicada
pub mod config;
/// Represents virtual filesystems which can be mounted to clients.
pub mod filesystem;
/// Nodes represent entries in a filesystem.
/// They represent either Files or Folders.
pub mod node;
/// Cicada users.
pub mod user;

#[typeshare(serialized_as = "number")]
pub type U64 = u64;
#[typeshare(serialized_as = "string")]
pub type Iso8601Timestamp = surrealdb_types::Datetime;

#[macro_export]
macro_rules! surreal_id {
  ($typ:ident, $table:expr) => {
    impl $typ {
      pub fn as_record_id(&self) -> RecordId {
        RecordId::new($table, self.0.as_str())
      }
    }

    impl SurrealValue for $typ {
      fn kind_of() -> surrealdb_types::Kind {
        surrealdb_types::Kind::Record(vec![])
      }

      fn into_value(self) -> surrealdb_types::Value {
        surrealdb_types::Value::RecordId(
          surrealdb_types::RecordId::new($table, self.0),
        )
      }

      fn from_value(
        value: surrealdb_types::Value,
      ) -> anyhow::Result<Self>
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
  };
}
