use axum::http::StatusCode;
use cicada_client::{
  api::write::policy::{CreatePolicy, UpdatePolicy},
  entities::policy::{PolicyId, PolicyRecord},
};
use mogh_error::AddStatusCode as _;
use mogh_error::anyhow::Context as _;

use crate::db::DB;

pub async fn list_all_policies()
-> mogh_error::Result<Vec<PolicyRecord>> {
  DB.select("Policy")
    .await
    .context("Failed to query for Policies")
    .map_err(Into::into)
}

pub async fn get_policy(
  id_or_name: String,
) -> mogh_error::Result<PolicyRecord> {
  DB.query(
    "
SELECT * FROM Policy
WHERE id = $id OR name = $name",
  )
  .bind(("id", PolicyId(id_or_name.clone())))
  .bind(("name", id_or_name))
  .await
  .context("Failed to query database")?
  .take::<Option<PolicyRecord>>(0)
  .context("Failed to get query result")?
  .context("Failed to find Secret with given parameters.")
  .status_code(StatusCode::NOT_FOUND)
}

pub async fn create_policy(
  body: CreatePolicy,
) -> mogh_error::Result<PolicyRecord> {
  DB.create("Policy")
    .content(body)
    .await
    .context("Failed to create Policy on database")?
    .context(
      "Failed to create Policy on database: No creation result",
    )
    .map_err(Into::into)
}

pub async fn update_policy(
  body: UpdatePolicy,
) -> mogh_error::Result<PolicyRecord> {
  DB.update(body.id.as_record_id())
    .merge(serde_json::to_value(body)?)
    .await
    .context("Failed to update Policy on database")?
    .context("Failed to update Policy on database: No update result")
    .status_code(StatusCode::NOT_FOUND)
}

pub async fn delete_policy(
  id: PolicyId,
) -> mogh_error::Result<PolicyRecord> {
  DB.query("DELETE Node WHERE policy = $policy RETURN NONE;")
    .bind(("Policy", id.clone()))
    .await
    .context("Failed to delete Policy nodes")?;
  DB.delete(id.as_record_id())
    .await?
    .context("No Policy matching given ID")
    .status_code(StatusCode::NOT_FOUND)
}
