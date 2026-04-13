use mogh_resolver::Resolve;
use serde::{Deserialize, Serialize};
use surrealdb_types::SurrealValue;
use typeshare::typeshare;

use crate::{
  api::read::CicadaReadRequest,
  entities::{
    encryption_key::EncryptionKeyEntity,
    filesystem::FilesystemRecord, node::NodeListItem,
    policy::PolicyRecord, secret::SecretListItem,
  },
};

//

#[cfg(feature = "utoipa")]
#[utoipa::path(
  post,
  path = "/read/Search",
  description = "Search resources matching a keyword",
  request_body(content = Search),
  responses(
    (status = 200, description = "List of search results", body = SearchResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn search() {}

/// Search resources. Response: [SearchResponse].
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[empty_traits(CicadaReadRequest)]
#[response(SearchResponse)]
#[error(mogh_error::Error)]
pub struct Search {
  /// The search keyword to match resources by name
  pub keyword: String,
}

/// Response for [Search].
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
#[surreal(crate = "surrealdb_types")]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SearchResponse {
  pub encryption_keys: Vec<EncryptionKeyEntity>,
  pub secrets: Vec<SecretListItem>,
  pub filesystems: Vec<FilesystemRecord>,
  pub nodes: Vec<NodeListItem>,
  pub policies: Vec<PolicyRecord>,
}
