use cicada_client::{
  api::read::*,
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
    encryption_keys().contains(&key.id.0)
  } else {
    true
  };
  EncryptionKeyEntity {
    id: key.id,
    name: key.name,
    kind: key.kind,
    created_at: key.created_at,
    updated_at: key.updated_at,
    initialized: Some(initialized),
  }
}

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
