use mogh_resolver::Resolve;
use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::{
  api::write::CicadaWriteRequest,
  entities::{
    encryption_key::EncryptionKeyId,
    secret::{SecretEntity, SecretId},
  },
};

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/CreateSecret",
  description = "Create a new secret",
  request_body(content = CreateSecret),
  responses(
    (status = 200, description = "The created secret", body = CreateSecretResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn create_secret() {}

/// Create secret. Response: [CreateSecretResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(CreateSecretResponse)]
#[error(mogh_error::Error)]
pub struct CreateSecret {
  /// The name of the secret
  pub name: String,
  /// An optional description for the secret
  #[serde(default)]
  pub description: String,
  /// The secret data.
  pub data: Option<String>,
  /// Choose a specific encryption key.
  /// Otherwise chooses the current global default.
  pub encryption_key: Option<EncryptionKeyId>,
}

/// Response for [CreateSecret].
#[typeshare]
pub type CreateSecretResponse = SecretEntity;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/UpdateSecret",
  description = "Update a secret",
  request_body(content = UpdateSecret),
  responses(
    (status = 200, description = "The updated secret", body = UpdateSecretResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn update_secret() {}

/// Update a secret. Response: [UpdateSecretResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(UpdateSecretResponse)]
#[error(mogh_error::Error)]
pub struct UpdateSecret {
  /// The secret id
  pub id: SecretId,
  /// The name of the secret
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  /// A description for the secret
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
}

/// Response for [UpdateSecret].
#[typeshare]
pub type UpdateSecretResponse = SecretEntity;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/UpdateSecretData",
  description = "Update a secret's data",
  request_body(content = UpdateSecret),
  responses(
    (status = 200, description = "The updated secret", body = UpdateSecretDataResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn update_secret_data() {}

/// Update a secret's encrypted data. Response: [UpdateSecretDataResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(UpdateSecretResponse)]
#[error(mogh_error::Error)]
pub struct UpdateSecretData {
  /// The secret id
  pub id: SecretId,
  /// The secret data
  pub data: String,
  /// Optionally update the encryption key used as master in the envelope encryption.
  pub encryption_key: Option<EncryptionKeyId>,
}

/// Response for [UpdateSecretData].
#[typeshare]
pub type UpdateSecretDataResponse = SecretEntity;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/UpdateSecretEncryptionKey",
  description = "Update a secret's data",
  request_body(content = UpdateSecret),
  responses(
    (status = 200, description = "The updated secret", body = UpdateSecretEncryptionKeyResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn update_secret_encryption_key() {}

/// Update a secret's encryption key. Response: [UpdateSecretEncryptionKeyResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(UpdateSecretEncryptionKeyResponse)]
#[error(mogh_error::Error)]
pub struct UpdateSecretEncryptionKey {
  /// The secret id
  pub id: SecretId,
  /// Update the encryption key used as master in the envelope encryption.
  pub encryption_key: EncryptionKeyId,
}

/// Response for [UpdateSecretEncryptionKey].
#[typeshare]
pub type UpdateSecretEncryptionKeyResponse = SecretEntity;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/RotateSecretEnvelopeKey",
  description = "Update a secret's data",
  request_body(content = UpdateSecret),
  responses(
    (status = 200, description = "The updated secret", body = RotateSecretEnvelopeKeyResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn rotate_secret_envelope_key() {}

/// Rotate a secret's envelope encryption key. Response: [RotateSecretEnvelopeKeyResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(RotateSecretEnvelopeKeyResponse)]
#[error(mogh_error::Error)]
pub struct RotateSecretEnvelopeKey {
  /// The secret id
  pub id: SecretId,
}

/// Response for [RotateSecretEnvelopeKey].
#[typeshare]
pub type RotateSecretEnvelopeKeyResponse = SecretEntity;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/DeleteSecret",
  description = "Delete a secret",
  request_body(content = DeleteSecret),
  responses(
    (status = 200, description = "The deleted secret", body = SecretEntity),
    (status = 404, description = "Secret not found", body = mogh_error::Serror),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn delete_secret() {}

/// Delete a secret. Response: [DeleteSecretResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(DeleteSecretResponse)]
#[error(mogh_error::Error)]
pub struct DeleteSecret {
  /// The secret id
  pub id: SecretId,
}

/// Response for [DeleteSecret].
#[typeshare]
pub type DeleteSecretResponse = SecretEntity;

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/write/BatchDeleteSecrets",
  description = "Batch delete many secrets recursively.",
  request_body(content = BatchDeleteSecrets),
  responses(
    (status = 200, description = "The deleted secrets", body = BatchDeleteSecretsResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn batch_delete_secrets() {}

/// Batch delete secrets. Response: [BatchDeleteSecretsResponse].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, SurrealValue, Resolve,
)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaWriteRequest)]
#[response(BatchDeleteSecretsResponse)]
#[error(mogh_error::Error)]
pub struct BatchDeleteSecrets {
  /// The onboarding_key ID
  pub ids: Vec<SecretId>,
}

/// Response for [BatchDeleteSecrets].
#[typeshare]
pub type BatchDeleteSecretsResponse = Vec<SecretEntity>;
