use mogh_auth_client::api::NoData;
use mogh_resolver::Resolve;
use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::{
  api::write::CicadaWriteRequest,
  entities::encryption_key::{
    EncryptionKeyEntity, EncryptionKeyId, EncryptionKeyKind, EncryptionKeyRecord
  },
};

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/CreateEncryptionKey",
  description = "Create a new encryption key",
  request_body(content = CreateEncryptionKey),
  responses(
    (status = 200, description = "The created encryption key", body = CreateEncryptionKeyResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn create_encryption_key() {}

/// Create an encryption key. Response: [CreateEncryptionKeyResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
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

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/UpdateEncryptionKey",
  description = "Update a encryption key",
  request_body(content = UpdateEncryptionKey),
  responses(
    (status = 200, description = "The updated encryption key", body = UpdateEncryptionKeyResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn update_encryption_key() {}

/// Update an encryption key. Response: [UpdateEncryptionKeyResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
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

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/InitializeEncryptionKey",
  description = "Initialize a encryption key",
  request_body(content = InitializeEncryptionKey),
  responses(
    (status = 200, description = "Encryption key initialized", body = InitializeEncryptionKeyResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn initialize_encryption_key() {}

/// Initialize an uninitialized in-memory encryption key.
/// Response: [InitializeEncryptionKeyResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
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

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/UninitializeEncryptionKey",
  description = "Uninitialize a encryption key",
  request_body(content = UninitializeEncryptionKey),
  responses(
    (status = 200, description = "Encryption key uninitialized", body = UninitializeEncryptionKeyResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn uninitialize_encryption_key() {}

/// Uninitialize an in-memory encryption key after it has been initialized.
/// Response: [UninitializeEncryptionKeyResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(UninitializeEncryptionKeyResponse)]
#[error(mogh_error::Error)]
pub struct UninitializeEncryptionKey {
  /// The encryption key ID
  pub id: EncryptionKeyId,
}

/// Response for [UninitializeEncryptionKey].
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UninitializeEncryptionKeyResponse {
  /// Whether an initialized encryption key was removed.
  /// It may be `false` because:
  /// - The encryption key was not initialized.
  /// - There is no encryption key at id.
  pub removed: bool,
}

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/DeleteEncryptionKey",
  description = "Delete an encryption key",
  request_body(content = DeleteEncryptionKey),
  responses(
    (status = 200, description = "The deleted encryption key", body = EncryptionKeyEntity),
    (status = 404, description = "Encryption Key not found", body = mogh_error::Serror),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn delete_encryption_key() {}

/// Delete an encryption key. Response: [DeleteEncryptionKeyResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(DeleteEncryptionKeyResponse)]
#[error(mogh_error::Error)]
pub struct DeleteEncryptionKey {
  /// The encryption key id
  pub id: EncryptionKeyId,
}

/// Response for [DeleteEncryptionKey].
#[typeshare]
pub type DeleteEncryptionKeyResponse = EncryptionKeyEntity;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/BatchDeleteEncryptionKeys",
  description = "Batch delete many encryption keys.",
  request_body(content = BatchDeleteEncryptionKeys),
  responses(
    (status = 200, description = "The deleted encryption keys", body = BatchDeleteEncryptionKeysResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn batch_delete_encryption_keys() {}

/// Batch delete encryption_keys. Response: [BatchDeleteEncryptionKeysResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(BatchDeleteEncryptionKeysResponse)]
#[error(mogh_error::Error)]
pub struct BatchDeleteEncryptionKeys {
  /// The onboarding_key ID
  pub ids: Vec<EncryptionKeyId>,
}

/// Response for [BatchDeleteEncryptionKeys].
#[typeshare]
pub type BatchDeleteEncryptionKeysResponse = Vec<EncryptionKeyEntity>;
