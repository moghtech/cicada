use std::collections::HashSet;

use cicada_client::api::read::Search;
use mogh_resolver::Resolve;

use crate::{
  api::read::ReadArgs,
  db::query::{self, policy::list_policies_for_client},
};

impl Resolve<ReadArgs> for Search {
  async fn resolve(
    self,
    ReadArgs { client }: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    let mut res = query::search::search(self.keyword).await?;

    if client.is_admin_user() {
      return Ok(res);
    }

    // Apply filesystem permissioning
    let filesystem_ids = list_policies_for_client(client)
      .await?
      .into_iter()
      .flat_map(|p| p.filesystems)
      .collect::<HashSet<_>>();

    res.filesystems.retain(|fs| filesystem_ids.contains(&fs.id));
    res
      .nodes
      .retain(|node| filesystem_ids.contains(&node.filesystem));
    // Non admins can't query policies
    res.policies.clear();

    Ok(res)
  }
}
