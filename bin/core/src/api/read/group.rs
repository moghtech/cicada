use cicada_client::api::read::ListGroups;
use mogh_resolver::Resolve;

use crate::{api::read::ReadArgs, db::query};

impl Resolve<ReadArgs> for ListGroups {
  async fn resolve(
    self,
    ReadArgs { client: _client }: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::group::list_groups().await
  }
}
