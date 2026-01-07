use std::time::Instant;

use axum::{Router, extract::Path, routing::post};
use cicada_client::api::read::{
  GetVersion, GetVersionResponse,
  filesystem::ListFilesystems,
  node::{FindNode, GetNode, ListNodes},
};
use mogh_error::Json;
use resolver_api::Resolve;
use serde::{Deserialize, Serialize};
use serde_json::json;
use surrealdb::types::Uuid;
use typeshare::typeshare;

use crate::{
  api::{Variant, response::Response},
  auth::middleware::auth_request,
};

pub mod filesystem;
pub mod node;

pub struct ReadArgs {}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
#[args(ReadArgs)]
#[response(Response)]
#[error(mogh_error::Error)]
#[serde(tag = "type", content = "params")]
enum ReadRequest {
  GetVersion(GetVersion),

  // ==== FILESYSTEM ====
  ListFilesystems(ListFilesystems),

  // ==== NODE ====
  ListNodes(ListNodes),
  GetNode(GetNode),
  FindNode(FindNode),
}

pub fn router() -> Router {
  Router::new()
    .route("/", post(handler))
    .route("/{variant}", post(variant_handler))
    .layer(axum::middleware::from_fn(auth_request))
}

async fn variant_handler(
  // user: Extension<User>,
  Path(Variant { variant }): Path<Variant>,
  Json(params): Json<serde_json::Value>,
) -> mogh_error::Result<axum::response::Response> {
  let req: ReadRequest = serde_json::from_value(json!({
    "type": variant,
    "params": params,
  }))?;
  handler(Json(req)).await
}

async fn handler(
  // Extension(user): Extension<User>,
  Json(request): Json<ReadRequest>,
) -> mogh_error::Result<axum::response::Response> {
  let timer = Instant::now();
  let req_id = Uuid::new_v4();
  // debug!("/read request | user: {}", user.username);
  let res = request.resolve(&ReadArgs {}).await;
  // if let Err(e) = &res {
  //   debug!("/read request {req_id} error: {:#}", e.error);
  // }
  let elapsed = timer.elapsed();
  debug!("/read request {req_id} | resolve time: {elapsed:?}");
  res.map(|res| res.0)
}

//

#[utoipa::path(
  post,
  path = "/read/GetVersion",
  description = "Get the Cicada Core version",
  request_body(content = GetVersion),
  responses(
    (status = 200, description = "Cicada Core version", body = GetVersionResponse),
  ),
)]
fn get_version() -> mogh_error::Result<GetVersionResponse> {
  Ok(GetVersionResponse {
    version: env!("CARGO_PKG_VERSION").to_string(),
  })
}

impl Resolve<ReadArgs> for GetVersion {
  async fn resolve(
    self,
    _: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    get_version()
  }
}
