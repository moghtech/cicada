use axum::http::StatusCode;
use cicada_client::{
  api::write::{CreatePolicy, UpdatePolicy},
  entities::policy::{PolicyId, PolicyRecord},
};
use mogh_error::anyhow::Context as _;
use mogh_error::{AddStatusCode as _, AddStatusCodeError as _};

use crate::{auth::middleware::Client, db::DB};

pub async fn list_all_policies()
-> mogh_error::Result<Vec<PolicyRecord>> {
  DB.select("Policy")
    .await
    .context("Failed to query for Policies")
    .map_err(Into::into)
}

pub async fn list_policies_for_client(
  client: &Client,
) -> mogh_error::Result<Vec<PolicyRecord>> {
  let (id_field, id, groups) = match client {
    Client::User(user) => {
      ("users", user.id.as_record_id(), &user.groups)
    }
    Client::Device(device) => {
      ("devices", device.id.as_record_id(), &device.groups)
    }
    Client::OnboardingKey(_) => {
      return Err(
        mogh_error::anyhow::anyhow!(
          "OnboardingKey clients do not have policies"
        )
        .status_code(StatusCode::FORBIDDEN),
      );
    }
  };
  DB
    .query(format!(
      "SELECT * FROM Policy WHERE enabled = true AND ({id_field} CONTAINS $id OR groups CONTAINSANY $groups)"
    ))
    .bind(("id", id))
    .bind(("groups", groups.clone()))
    .await
    .context("Failed to query policies")?
    .take::<Vec<PolicyRecord>>(0)
    .context("Failed to get policy query result")
    .map_err(Into::into)
}

pub async fn get_policy(
  id_or_name: String,
) -> mogh_error::Result<PolicyRecord> {
  DB.query("SELECT * FROM Policy WHERE id = $id OR name = $name")
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
  DB.query("UPDATE $id MERGE fn::object_strip_none($body);")
    .bind(("id", body.id.clone()))
    .bind(("body", body))
    .await
    .context("Failed to query database")?
    .take::<Option<PolicyRecord>>(0)
    .context("Failed to get query result")?
    .context("Failed to find policy with given parameters.")
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

pub async fn batch_delete_policies(
  ids: Vec<PolicyId>,
) -> mogh_error::Result<Vec<PolicyRecord>> {
  DB.query("DELETE Policy WHERE id IN $ids RETURN BEFORE;")
    .bind(("ids", ids))
    .await
    .context("Failed to delete policies")?
    .take(0)
    .context("Invalid delete policies query response")
    .map_err(Into::into)
}
