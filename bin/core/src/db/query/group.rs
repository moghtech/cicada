use cicada_client::entities::group::GroupEntity;
use mogh_error::anyhow::Context as _;

use crate::db::DB;

pub async fn list_groups() -> mogh_error::Result<Vec<GroupEntity>> {
  DB.query("fn::list_groups()")
    .await
    .context("Failed to query for Groups")?
    .take(0)
    .context("Failed to get Group query result")
    .map_err(Into::into)
}
