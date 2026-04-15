use cicada_client::api::write::{BatchDeleteApiKeys, UpdateApiKey};
use mogh_resolver::Resolve;

use crate::{api::write::WriteArgs, db::query};

//

impl Resolve<WriteArgs> for UpdateApiKey {
  async fn resolve(
    self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    client.admin_only()?;
    query::api_key::update_api_key(self).await
  }
}

//

impl Resolve<WriteArgs> for BatchDeleteApiKeys {
  async fn resolve(
    self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    client.admin_only()?;
    query::api_key::batch_delete_api_keys(self.ids).await
  }
}
