use anyhow::Context as _;
use axum::{Extension, Router, extract::Path, routing::post};
use cicada_client::api::write::{device::*, filesystem::*, node::*};
use derive_variants::{EnumVariants, ExtractVariant as _};
use mogh_error::{Json, Response};
use resolver_api::Resolve;
use serde::{Deserialize, Serialize};
use serde_json::json;
use strum::Display;
use surrealdb::types::Uuid;
use typeshare::typeshare;

use crate::{
  api::Variant,
  auth::middleware::{Client, auth_request},
};

pub mod device;
pub mod filesystem;
pub mod node;

pub struct WriteArgs {
  _client: Client,
}

#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, Resolve, EnumVariants,
)]
#[variant_derive(Debug, Display)]
#[args(WriteArgs)]
#[response(Response)]
#[error(mogh_error::Error)]
#[serde(tag = "type", content = "params")]
pub enum WriteRequest {
  // ==== DEVICE ====
  CreateDevice(CreateDevice),
  UpdateDevice(UpdateDevice),
  DeleteDevice(DeleteDevice),

  // ==== FILESYSTEM ====
  CreateFilesystem(CreateFilesystem),
  UpdateFilesystem(UpdateFilesystem),
  DeleteFilesystem(DeleteFilesystem),

  // ==== NODE ====
  CreateNode(CreateNode),
  UpdateNode(UpdateNode),
  DeleteNode(DeleteNode),
}

pub fn router() -> Router {
  Router::new()
    .route("/", post(handler))
    .route("/{variant}", post(variant_handler))
    .layer(axum::middleware::from_fn(auth_request))
}

async fn variant_handler(
  client: Extension<Client>,
  Path(Variant { variant }): Path<Variant>,
  Json(params): Json<serde_json::Value>,
) -> mogh_error::Result<axum::response::Response> {
  let req: WriteRequest = serde_json::from_value(json!({
    "type": variant,
    "params": params,
  }))?;
  handler(client, Json(req)).await
}

async fn handler(
  Extension(client): Extension<Client>,
  Json(request): Json<WriteRequest>,
) -> mogh_error::Result<axum::response::Response> {
  let req_id = Uuid::new_v4();

  let res = tokio::spawn(task(req_id, request, client))
    .await
    .context("failure in spawned task");

  res?
}

/// Spawn a task to handle write requests
/// to ensure they finish even if client disconnects.
async fn task(
  req_id: Uuid,
  request: WriteRequest,
  _client: Client,
) -> mogh_error::Result<axum::response::Response> {
  let variant = request.extract_variant();
  info!("/write request | {variant}");

  let res = request.resolve(&WriteArgs { _client }).await;

  if let Err(e) = &res {
    warn!(
      "/write request {req_id} | {variant} | error: {:#}",
      e.error
    );
  }

  res.map(|res| res.0)
}
