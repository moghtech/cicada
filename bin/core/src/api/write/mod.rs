use anyhow::Context as _;
use axum::{Router, extract::Path, routing::post};
use cicada_client::api::write::{
  filesystem::{CreateFilesystem, UpdateFilesystem},
  node::{CreateNode, UpdateNode},
};
use derive_variants::{EnumVariants, ExtractVariant as _};
use resolver_api::Resolve;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serror::Json;
use strum::Display;
use surrealdb::types::Uuid;
use typeshare::typeshare;

use crate::api::{Variant, response::Response};

mod filesystem;
mod node;

pub struct WriteArgs {}

#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, Resolve, EnumVariants,
)]
#[variant_derive(Debug, Display)]
#[args(WriteArgs)]
#[response(Response)]
#[error(serror::Error)]
#[serde(tag = "type", content = "params")]
pub enum WriteRequest {
  // ==== NODE ====
  CreateFilesystem(CreateFilesystem),
  UpdateFilesystem(UpdateFilesystem),

  // ==== NODE ====
  CreateNode(CreateNode),
  UpdateNode(UpdateNode),
}

pub fn router() -> Router {
  Router::new()
    .route("/", post(handler))
    .route("/{variant}", post(variant_handler))
  // .layer(middleware::from_fn(auth_request))
}

async fn variant_handler(
  // user: Extension<User>,
  Path(Variant { variant }): Path<Variant>,
  Json(params): Json<serde_json::Value>,
) -> serror::Result<axum::response::Response> {
  let req: WriteRequest = serde_json::from_value(json!({
    "type": variant,
    "params": params,
  }))?;
  handler(Json(req)).await
}

async fn handler(
  // Extension(user): Extension<User>,
  Json(request): Json<WriteRequest>,
) -> serror::Result<axum::response::Response> {
  let req_id = Uuid::new_v4();

  let res = tokio::spawn(task(req_id, request))
    .await
    .context("failure in spawned task");

  res?
}

async fn task(
  req_id: Uuid,
  request: WriteRequest,
  // user: User,
) -> serror::Result<axum::response::Response> {
  let variant = request.extract_variant();
  info!("/write request | {variant}");

  let res = request.resolve(&WriteArgs {}).await;

  if let Err(e) = &res {
    warn!(
      "/write request {req_id} | {variant} | error: {:#}",
      e.error
    );
  }

  res.map(|res| res.0)
}
