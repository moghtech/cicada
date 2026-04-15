use cicada_client::api::read::ListApiKeys;
use mogh_resolver::Resolve;

use crate::{api::read::ReadArgs, db::query};

impl Resolve<ReadArgs> for ListApiKeys {
  async fn resolve(
    self,
    ReadArgs { client }: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    client.admin_only()?;
    query::api_key::list_api_keys(client.as_user()?.id.clone()).await
  }
}
