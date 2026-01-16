use cicada_client::{
  api::write::encryption_key::{
    CreateEncryptionKey, UpdateEncryptionKey,
  },
  entities::encryption_key::{
    EncryptionKeyKind, EncryptionKeyRecord,
  },
};
use mogh_encryption::BASE64URL;
use resolver_api::Resolve;

use crate::{api::write::WriteArgs, db::query};

#[allow(unused)]
#[utoipa::path(
  post,
  path = "/write/CreateEncryptionKey",
  description = "Create a new encryption key",
  request_body(content = CreateEncryptionKey),
  responses(
    (status = 200, description = "The created encryption key", body = EncryptionKeyRecord),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn create_encryption_key() {}

impl Resolve<WriteArgs> for CreateEncryptionKey {
  async fn resolve(
    mut self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    match self.kind {
      EncryptionKeyKind::Memory => self.key = None,
      EncryptionKeyKind::Disk => {
        if self.key.is_none() {
          self.key =
            BASE64URL.encode(&rand::random::<[u8; 32]>()).into();
        }
      }
    }
    query::encryption_key::create_encryption_key(self)
      .await
      .map_err(Into::into)
  }
}

//

#[allow(unused)]
#[utoipa::path(
  post,
  path = "/write/UpdateEncryptionKey",
  description = "Update a encryption key",
  request_body(content = UpdateEncryptionKey),
  responses(
    (status = 200, description = "The updated encryption key", body = EncryptionKeyRecord),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn update_encryption_key() {}

impl Resolve<WriteArgs> for UpdateEncryptionKey {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::encryption_key::update_encryption_key(self)
      .await
      .map_err(Into::into)
  }
}
