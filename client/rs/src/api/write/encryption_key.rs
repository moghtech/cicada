use derive_empty_traits::EmptyTraits;
use mogh_auth_client::api::NoData;
use resolver_api::Resolve;
use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::{
  api::write::CicadaWriteRequest,
  entities::encryption_key::{
    EncryptionKeyId, EncryptionKeyKind, EncryptionKeyRecord,
  },
};

//

/// Create an encryption key. Response: [CreateEncryptionKeyResponse].
#[typeshare]
#[derive(
  Debug,
  Clone,
  Serialize,
  Deserialize,
  SurrealValue,
  Resolve,
  EmptyTraits,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(CreateEncryptionKeyResponse)]
#[error(mogh_error::Error)]
pub struct CreateEncryptionKey {
  /// The name of the encryption key
  pub name: String,
  /// The kind of encryption key
  pub kind: EncryptionKeyKind,
  /// Disk mode only. If not provided in Disk mode,
  /// one will be generated.
  pub key: Option<String>,
}

/// Response for [CreateEncryptionKey].
#[typeshare]
pub type CreateEncryptionKeyResponse = EncryptionKeyRecord;

//

/// Update an encryption key. Response: [UpdateEncryptionKeyResponse].
#[typeshare]
#[derive(
  Debug,
  Clone,
  Serialize,
  Deserialize,
  SurrealValue,
  Resolve,
  EmptyTraits,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(UpdateEncryptionKeyResponse)]
#[error(mogh_error::Error)]
pub struct UpdateEncryptionKey {
  /// The encryption key ID
  pub id: EncryptionKeyId,
  /// The name of the encryption key
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
}

/// Response for [UpdateEncryptionKey].
#[typeshare]
pub type UpdateEncryptionKeyResponse = EncryptionKeyRecord;

//

/// Initialize an in-memory encryption key after application startup.
/// Response: [InitializeEncryptionKeyResponse].
#[typeshare]
#[derive(
  Debug,
  Clone,
  Serialize,
  Deserialize,
  SurrealValue,
  Resolve,
  EmptyTraits,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(InitializeEncryptionKeyResponse)]
#[error(mogh_error::Error)]
pub struct InitializeEncryptionKey {
  /// The encryption key ID
  pub id: EncryptionKeyId,
  /// base64url encoded master key
  pub key: String,
}

/// Response for [InitializeEncryptionKey].
#[typeshare]
pub type InitializeEncryptionKeyResponse = NoData;
