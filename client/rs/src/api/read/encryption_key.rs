use mogh_resolver::Resolve;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
  api::read::CicadaReadRequest,
  entities::encryption_key::{EncryptionKeyEntity, EncryptionKeyId},
};

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/read/ListEncryptionKeys",
  description = "List available encryption keys.",
  request_body(content = ListEncryptionKeys),
  responses(
    (status = 200, description = "List of encryption keys", body = ListEncryptionKeysResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn list_encryption_keys() {}

/// List encryption keys. Response: [ListEncryptionKeysResponse].
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaReadRequest)]
#[response(ListEncryptionKeysResponse)]
#[error(mogh_error::Error)]
pub struct ListEncryptionKeys {}

/// Response for [ListEncryptionKeys].
#[typeshare]
pub type ListEncryptionKeysResponse = Vec<EncryptionKeyEntity>;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/read/GetEncryptionKey",
  description = "Get an encryption key by id",
  request_body(content = GetEncryptionKey),
  responses(
    (status = 200, description = "The encryption key", body = GetEncryptionKeyResponse),
    (status = 404, description = "Failed to find encryption key with given id", body = mogh_error::Serror),
    (status = 500, description = "Request failed", body = mogh_error::Serror),
  ),
)]
pub fn get_encryption_key() {}

/// Get an encryption key. Response: [EncryptionKeyEntity].
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
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
