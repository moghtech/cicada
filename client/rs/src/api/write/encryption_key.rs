use derive_empty_traits::EmptyTraits;
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
