use cicada_client::api::write::policy::*;
use mogh_resolver::Resolve;

use crate::{api::write::WriteArgs, db::query};

//

impl Resolve<WriteArgs> for CreatePolicy {
  async fn resolve(
    self,
    WriteArgs { client: _client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::policy::create_policy(self).await
  }
}

//

impl Resolve<WriteArgs> for UpdatePolicy {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::policy::update_policy(self).await
  }
}

//

impl Resolve<WriteArgs> for DeletePolicy {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::policy::delete_policy(self.id).await
  }
}
