use cicada_client::{
  api::read::encryption_key::ListEncryptionKeys,
  entities::encryption_key::EncryptionKeyRecord,
};
use resolver_api::Resolve;

use crate::{
  api::read::ReadArgs,
  db::query::encryption_key::list_all_encryption_keys,
};

#[allow(unused)]
#[utoipa::path(
  post,
  path = "/read/ListEncryptionKeys",
  description = "List available encryption keys",
  request_body(content = ListEncryptionKeys),
  responses(
    (status = 200, description = "List of encryption keys", body = Vec<EncryptionKeyRecord>),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn list_encryption_keys() {}

impl Resolve<ReadArgs> for ListEncryptionKeys {
  async fn resolve(
    self,
    _: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    list_all_encryption_keys().await.map_err(Into::into)
  }
}
