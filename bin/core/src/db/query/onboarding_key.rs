use axum::http::StatusCode;
use cicada_client::{
  api::write::onboarding_key::UpdateOnboardingKey,
  entities::{
    Iso8601Timestamp,
    onboarding_key::{OnboardingKeyId, OnboardingKeyRecord},
  },
};
use mogh_error::AddStatusCode as _;
use mogh_error::anyhow::Context as _;
use surrealdb_types::SurrealValue;

use crate::db::DB;

pub async fn list_all_onboarding_keys()
-> mogh_error::Result<Vec<OnboardingKeyRecord>> {
  DB.select("OnboardingKey")
    .await
    .context("Failed to query for OnboardingKeys")
    .map_err(Into::into)
}

pub async fn get_onboarding_key(
  onboarding_key_id: &str,
) -> mogh_error::Result<OnboardingKeyRecord> {
  DB.select::<Option<OnboardingKeyRecord>>((
    "OnboardingKey",
    onboarding_key_id,
  ))
  .await
  .context("Failed to query database for onboarding_key")?
  .context("No onboarding_key found with given ID")
  .status_code(StatusCode::NOT_FOUND)
}

pub async fn find_onboarding_key_with_public_key(
  public_key: String,
) -> mogh_error::Result<Option<OnboardingKeyRecord>> {
  let onboarding_key = DB
    .query(
      "SELECT * FROM OnboardingKey WHERE public_key = $public_key",
    )
    .bind(("public_key", public_key))
    .await
    .context("Failed to query database for onboarding key")?
    .take::<Vec<OnboardingKeyRecord>>(0)
    .context("Failed to deserialize OnboardingKeyRecord")?
    .pop();
  Ok(onboarding_key)
}

#[derive(SurrealValue)]
pub struct CreateOnboardingKeyQuery {
  pub name: String,
  pub public_key: String,
  pub enabled: bool,
  pub expires: Option<Iso8601Timestamp>,
}

pub async fn create_onboarding_key(
  query: CreateOnboardingKeyQuery,
) -> mogh_error::Result<OnboardingKeyRecord> {
  DB.create("OnboardingKey")
    .content(query)
    .await
    .context("Failed to create OnboardingKey on database")?
    .context(
      "Failed to create OnboardingKey on database: No creation result",
    )
    .map_err(Into::into)
}

pub async fn update_onboarding_key(
  body: UpdateOnboardingKey,
) -> mogh_error::Result<OnboardingKeyRecord> {
  DB.update(body.id.as_record_id())
    .merge(serde_json::to_value(body)?)
    .await
    .context("Failed to update OnboardingKey on database")?
    .context(
      "Failed to update OnboardingKey on database: No update result",
    )
    .status_code(StatusCode::NOT_FOUND)
}

pub async fn delete_onboarding_key(
  id: String,
) -> mogh_error::Result<OnboardingKeyRecord> {
  DB.delete(("OnboardingKey", id))
    .await?
    .context("No OnboardingKey matching given ID")
    .status_code(StatusCode::NOT_FOUND)
}

pub async fn batch_delete_onboarding_keys(
  ids: Vec<OnboardingKeyId>,
) -> mogh_error::Result<Vec<OnboardingKeyRecord>> {
  DB.query("DELETE OnboardingKey WHERE in IN $ids RETURN BEFORE;")
    .bind(("ids", ids))
    .await
    .context("Failed to delete onboarding keys")?
    .take(0)
    .context("Invalid delete onboarding keys query response")
    .map_err(Into::into)
}
