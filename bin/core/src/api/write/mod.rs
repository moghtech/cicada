use anyhow::Context as _;
use axum::{Extension, Router, extract::Path, routing::post};
use cicada_client::api::write::{
  device::*, encryption_key::*, filesystem::*, node::*,
  onboarding_key::*,
};
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
pub mod encryption_key;
pub mod filesystem;
pub mod node;
pub mod onboarding_key;

pub struct WriteArgs {
  client: Client,
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
  BatchDeleteDevices(BatchDeleteDevices),

  // ==== ONBOARDING KEY ====
  CreateOnboardingKey(CreateOnboardingKey),
  UpdateOnboardingKey(UpdateOnboardingKey),
  DeleteOnboardingKey(DeleteOnboardingKey),
  BatchDeleteOnboardingKeys(BatchDeleteOnboardingKeys),

  // ==== FILESYSTEM ====
  CreateFilesystem(CreateFilesystem),
  UpdateFilesystem(UpdateFilesystem),
  DeleteFilesystem(DeleteFilesystem),

  // ==== NODE ====
  CreateNode(CreateNode),
  UpdateNode(UpdateNode),
  UpdateNodeData(UpdateNodeData),
  UpdateNodeEncryptionKey(UpdateNodeEncryptionKey),
  RotateNodeEnvelopeKey(RotateNodeEnvelopeKey),
  DeleteNode(DeleteNode),
  BatchDeleteNodes(BatchDeleteNodes),

  // ==== ENCRYPTION KEY ====
  CreateEncryptionKey(CreateEncryptionKey),
  UpdateEncryptionKey(UpdateEncryptionKey),
  InitializeEncryptionKey(InitializeEncryptionKey),
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
  // Most of the write API is only for user clients.
  // This blocks non user access apart from device routes.
  if !matches!(
    &request,
    WriteRequest::CreateDevice(_) | WriteRequest::UpdateDevice(_)
  ) {
    client.only_users()?;
  }

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
  client: Client,
) -> mogh_error::Result<axum::response::Response> {
  let variant = request.extract_variant();
  info!("/write request | {variant}");

  let res = request.resolve(&WriteArgs { client }).await;

  if let Err(e) = &res {
    warn!(
      "/write request {req_id} | {variant} | error: {:#}",
      e.error
    );
  }

  res.map(|res| res.0)
}
