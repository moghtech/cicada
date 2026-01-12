use anyhow::Context as _;
use axum::http::StatusCode;
use cicada_client::{
  api::write::onboarding_key::{
    CreateOnboardingKey, UpdateOnboardingKey,
  },
  entities::onboarding_key::OnboardingKeyRecord,
};
use mogh_error::AddStatusCode as _;

use crate::db::DB;

pub async fn list_all_onboarding_keys()
-> anyhow::Result<Vec<OnboardingKeyRecord>> {
  DB.select("OnboardingKey")
    .await
    .context("Failed to query for OnboardingKeys")
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
) -> anyhow::Result<Option<OnboardingKeyRecord>> {
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

pub async fn create_onboarding_key(
  body: CreateOnboardingKey,
) -> anyhow::Result<OnboardingKeyRecord> {
  DB.create("OnboardingKey")
    .content(body)
    .await
    .context("Failed to create OnboardingKey on database")?
    .context(
      "Failed to create OnboardingKey on database: No creation result",
    )
}

pub async fn update_onboarding_key(
  body: UpdateOnboardingKey,
) -> anyhow::Result<OnboardingKeyRecord> {
  DB.update(body.id.as_record_id())
    .merge(serde_json::to_value(body)?)
    .await
    .context("Failed to update OnboardingKey on database")?
    .context(
      "Failed to update OnboardingKey on database: No update result",
    )
}

pub async fn delete_onboarding_key(
  id: String,
) -> mogh_error::Result<OnboardingKeyRecord> {
  DB.delete(("OnboardingKey", id))
    .await?
    .context("No OnboardingKey matching given ID")
    .status_code(StatusCode::NOT_FOUND)
}
