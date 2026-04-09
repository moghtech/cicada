use cicada_client::api::read::{GetPolicy, ListPolicies};
use mogh_resolver::Resolve;

use crate::{
  api::read::ReadArgs,
  db::query::{self, policy::list_all_policies},
};

impl Resolve<ReadArgs> for ListPolicies {
  async fn resolve(
    self,
    ReadArgs { client }: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    client.admin_only()?;
    list_all_policies().await
  }
}

impl Resolve<ReadArgs> for GetPolicy {
  async fn resolve(
    self,
    ReadArgs { client }: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    client.admin_only()?;
    query::policy::get_policy(self.id).await
  }
}
