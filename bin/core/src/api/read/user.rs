use cicada_client::api::read::ListUsers;
use mogh_resolver::Resolve;

use crate::{api::read::ReadArgs, db::query::user::list_all_users};

impl Resolve<ReadArgs> for ListUsers {
  async fn resolve(
    self,
    ReadArgs { client }: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    client.admin_only()?;
    list_all_users().await
  }
}
