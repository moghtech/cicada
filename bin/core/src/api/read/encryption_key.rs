use cicada_client::{
  api::read::encryption_key::*,
  entities::encryption_key::{
    EncryptionKeyEntity, EncryptionKeyKind, EncryptionKeyRecord,
  },
};
use mogh_resolver::Resolve;

use crate::{
  api::read::ReadArgs, db::query, encryption::encryption_keys,
};

fn convert_key(key: EncryptionKeyRecord) -> EncryptionKeyEntity {
  let initialized = if matches!(key.kind, EncryptionKeyKind::Memory) {
    encryption_keys().contains_key(&key.id.0)
  } else {
    true
  };
  EncryptionKeyEntity {
    id: key.id,
    name: key.name,
    kind: key.kind,
    created_at: key.created_at,
    updated_at: key.updated_at,
    initialized,
  }
}

#[allow(unused)]
#[utoipa::path(
  post,
  path = "/read/ListEncryptionKeys",
  description = "List available encryption keys",
  request_body(content = ListEncryptionKeys),
  responses(
    (status = 200, description = "List of encryption keys", body = ListEncryptionKeysResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn list_encryption_keys() {}

impl Resolve<ReadArgs> for ListEncryptionKeys {
  async fn resolve(
    self,
    _: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    let keys = query::encryption_key::list_all_encryption_keys()
      .await?
      .into_iter()
      .map(convert_key)
      .collect();
    Ok(keys)
  }
}

//

#[allow(unused)]
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
pub fn get_encryption_key(body: GetEncryptionKey) {}

impl Resolve<ReadArgs> for GetEncryptionKey {
  async fn resolve(
    self,
    _: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    let key =
      query::encryption_key::get_encryption_key(&self.id.0).await?;
    Ok(convert_key(key))
  }
}
