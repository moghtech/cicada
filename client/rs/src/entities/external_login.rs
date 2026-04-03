use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};
use surrealdb_types::{SerializationError, SurrealValue};
use typeshare::typeshare;

use crate::entities::{Iso8601Timestamp, user::UserId};

/// Stores external user logins
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ExternalLoginRecord {
  /// The unique user login id
  pub id: ExternalLoginId,
  /// The user which this method logs in
  pub user: UserId,
  /// The type of login.
  /// - **Oidc**
  /// - **Github**
  /// - **Google**
  pub kind: ExternalLoginKind,
  /// The login method external id.
  /// - **Oidc**: The OIDC user subject identifier
  /// - **Github**: The Github user id
  /// - **Google**: The Google user id
  pub external_id: String,
  // ===============
  // = TIMESTAMPS =
  // ===============
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
pub struct ExternalLoginId(pub String);

crate::surreal_id!(ExternalLoginId, "ExternalLogin");

/// The available kinds external of user logins.
#[typeshare]
#[derive(
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  Hash,
  Serialize,
  Deserialize,
  Display,
  EnumString,
  AsRefStr,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ExternalLoginKind {
  Oidc,
  Github,
  Google,
}

impl SurrealValue for ExternalLoginKind {
  fn kind_of() -> surrealdb_types::Kind {
    surrealdb_types::Kind::String
  }

  fn into_value(self) -> surrealdb_types::Value {
    surrealdb_types::Value::String(self.to_string())
  }

  fn from_value(
    value: surrealdb_types::Value,
  ) -> Result<Self, surrealdb_types::Error>
  where
    Self: Sized,
  {
    let surrealdb_types::Value::String(kind) = value else {
      return Err(surrealdb_types::Error::serialization(
        String::from("Value is not String"),
        SerializationError::Deserialization,
      ));
    };
    kind.parse().map_err(|e| {
      surrealdb_types::Error::serialization(
        format!("Invalid ExternalLoginKind: {e:?}"),
        SerializationError::Deserialization,
      )
    })
  }
}
