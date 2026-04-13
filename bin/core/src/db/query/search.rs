use cicada_client::api::read::SearchResponse;
use mogh_error::{AddStatusCode as _, StatusCode, anyhow::Context as _};

use crate::db::DB;

pub async fn search(
  keyword: String,
) -> mogh_error::Result<SearchResponse> {
  DB.query("fn::search($keyword);")
    .bind(("keyword", keyword))
    .await
    .context("Failed to query database")?
    .take::<Option<SearchResponse>>(0)
    .context("Failed to get query result")?
    .context("Failed to search resources with given parameters.")
    .status_code(StatusCode::NOT_FOUND)
}
