use derive_empty_traits::EmptyTraits;
use resolver_api::Resolve;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
  api::read::CicadaReadRequest,
  entities::encryption_key::EncryptionKeyRecord,
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
pub type ListEncryptionKeysResponse = Vec<EncryptionKeyRecord>;
