use cicada_client::api::write::filesystem::*;
use mogh_resolver::Resolve;

use crate::{api::write::WriteArgs, db::query};

impl Resolve<WriteArgs> for CreateFilesystem {
  async fn resolve(
    self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    let _user = client.as_user()?;
    query::filesystem::create_filesystem(self).await
  }
}

//

impl Resolve<WriteArgs> for UpdateFilesystem {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::filesystem::update_filesystem(self).await
  }
}

//

impl Resolve<WriteArgs> for DeleteFilesystem {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::filesystem::delete_filesystem(self.id.0).await
  }
}
