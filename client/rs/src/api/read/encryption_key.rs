use derive_empty_traits::EmptyTraits;
use resolver_api::Resolve;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
  api::read::CicadaReadRequest,
  entities::encryption_key::{EncryptionKeyEntity, EncryptionKeyId},
};

//

/// List encryption keys. Response: [ListEncryptionKeysResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, Resolve, EmptyTraits,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaReadRequest)]
#[response(ListEncryptionKeysResponse)]
#[error(mogh_error::Error)]
pub struct ListEncryptionKeys {}

/// Response for [ListEncryptionKeys].
#[typeshare]
pub type ListEncryptionKeysResponse = Vec<EncryptionKeyEntity>;

//

/// Get an encryption key. Response: [EncryptionKeyEntity].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, Resolve, EmptyTraits,
)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaReadRequest)]
#[response(GetEncryptionKeyResponse)]
#[error(mogh_error::Error)]
pub struct GetEncryptionKey {
  /// The encryption key id
  pub id: EncryptionKeyId,
}

/// Response for [GetEncryptionKey].
#[typeshare]
pub type GetEncryptionKeyResponse = EncryptionKeyEntity;
