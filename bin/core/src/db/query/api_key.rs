use cicada_client::{
  api::write::UpdateApiKey,
  entities::{
    Iso8601Timestamp,
    api_key::{ApiKeyId, ApiKeyRecord},
    user::UserId,
  },
};
use mogh_error::{AddStatusCode, StatusCode, anyhow::Context as _};
use surrealdb_types::SurrealValue;

use crate::db::DB;

pub async fn list_api_keys(
  user: UserId,
) -> mogh_error::Result<Vec<ApiKeyRecord>> {
  DB.query("fn::list_api_keys($user);")
    .bind(("user", user))
    .await
    .context("Failed to query database for api keys")?
    .take(0)
    .context("Failed to get api key query result")
    .map_err(Into::into)
}

pub async fn find_api_key(
  key: String,
) -> mogh_error::Result<ApiKeyRecord> {
  DB.query("SELECT * FROM ONLY ApiKey WHERE key = $key;")
    .bind(("key", key))
    .await
    .context("Failed to query database for api key")?
    .take::<Option<ApiKeyRecord>>(0)
    .context("Failed to deserialize ApiKeyRecord")?
    .context("Api key not found")
    .status_code(StatusCode::UNAUTHORIZED)
}

#[derive(SurrealValue)]
pub struct CreateApiKeyQuery {
  pub user: UserId,
  pub name: String,
  pub key: String,
  pub secret: String,
  pub enabled: bool,
  pub expires: Option<Iso8601Timestamp>,
}

pub async fn create_api_key(
  query: CreateApiKeyQuery,
) -> mogh_error::Result<ApiKeyRecord> {
  DB.create("ApiKey")
    .content(query)
    .await
    .context("Failed to create ApiKey on database")?
    .context(
      "Failed to create ApiKey on database: No creation result",
    )
    .map_err(Into::into)
}

pub async fn update_api_key(
  body: UpdateApiKey,
) -> mogh_error::Result<ApiKeyRecord> {
  DB.query("UPDATE $id MERGE fn::object_strip_none($body);")
    .bind(("id", body.id.clone()))
    .bind(("body", body))
    .await
    .context("Failed to query database")?
    .take::<Option<ApiKeyRecord>>(0)
    .context("Failed to get query result")?
    .context("Failed to find api key with given parameters.")
    .status_code(StatusCode::NOT_FOUND)
}

pub async fn delete_api_key_with_key(
  key: String,
) -> mogh_error::Result<ApiKeyRecord> {
  DB.query("DELETE ONLY ApiKey WHERE key = $key RETURN BEFORE;")
    .bind(("key", key))
    .await
    .context("Failed to query database for api key")?
    .take::<Option<ApiKeyRecord>>(0)
    .context("Failed to deserialize ApiKeyRecord")?
    .context("Api key not found")
    .status_code(StatusCode::UNAUTHORIZED)
}

pub async fn batch_delete_api_keys(
  ids: Vec<ApiKeyId>,
) -> mogh_error::Result<Vec<ApiKeyRecord>> {
  DB.query("DELETE ApiKey WHERE id IN $ids RETURN BEFORE;")
    .bind(("ids", ids))
    .await
    .context("Failed to delete api keys")?
    .take(0)
    .context("Invalid delete api keys query response")
    .map_err(Into::into)
}
