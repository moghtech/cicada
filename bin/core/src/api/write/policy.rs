use cicada_client::api::write::{
  BatchDeletePolicies, CreatePolicy, DeletePolicy, UpdatePolicy,
};
use mogh_resolver::Resolve;

use crate::{api::write::WriteArgs, db::query};

//

impl Resolve<WriteArgs> for CreatePolicy {
  async fn resolve(
    self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    client.admin_only()?;
    query::policy::create_policy(self).await
  }
}

//

impl Resolve<WriteArgs> for UpdatePolicy {
  async fn resolve(
    self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    client.admin_only()?;
    query::policy::update_policy(self).await
  }
}

//

impl Resolve<WriteArgs> for DeletePolicy {
  async fn resolve(
    self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    client.admin_only()?;
    query::policy::delete_policy(self.id).await
  }
}

//

impl Resolve<WriteArgs> for BatchDeletePolicies {
  async fn resolve(
    self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    client.admin_only()?;
    query::policy::batch_delete_policies(self.ids).await
  }
}
