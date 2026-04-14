use axum::{Extension, Router, extract::Path, routing::post};
use cicada_client::api::write::*;
use mogh_auth_server::middleware::authenticate_request;
use mogh_error::anyhow::Context as _;
use mogh_error::{Json, Response};
use mogh_resolver::Resolve;
use serde::{Deserialize, Serialize};
use serde_json::json;
use strum::{Display, EnumDiscriminants};
use surrealdb::types::Uuid;
use typeshare::typeshare;

use crate::auth::CicadaAuthImpl;
use crate::{api::Variant, auth::middleware::Client};

pub mod checkpoint;
pub mod device;
pub mod encryption_key;
pub mod filesystem;
pub mod node;
pub mod onboarding_key;
pub mod policy;
pub mod secret;
pub mod user;

pub struct WriteArgs {
  client: Client,
}

#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, Resolve, EnumDiscriminants,
)]
#[strum_discriminants(name(WriteRequestMethod), derive(Display))]
#[args(WriteArgs)]
#[response(Response)]
#[error(mogh_error::Error)]
#[serde(tag = "type", content = "params")]
pub enum WriteRequest {
  // ==== USER ====
  CreateUser(CreateUser),
  UpdateUser(UpdateUser),
  DeleteUser(DeleteUser),
  BatchDeleteUsers(BatchDeleteUsers),

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
  BatchDeleteFilesystems(BatchDeleteFilesystems),

  // ==== NODE ====
  CreateNode(CreateNode),
  UpdateNode(UpdateNode),
  UpdateNodeData(UpdateNodeData),
  UpdateNodeEncryptionKey(UpdateNodeEncryptionKey),
  RotateNodeEnvelopeKey(RotateNodeEnvelopeKey),
  MoveNode(MoveNode),
  DeleteNode(DeleteNode),
  BatchDeleteNodes(BatchDeleteNodes),

  // ==== CHECKPOINT ====
  UpdateCheckpoint(UpdateCheckpoint),
  UpdateCheckpointEncryptionKey(UpdateCheckpointEncryptionKey),
  RotateCheckpointEnvelopeKey(RotateCheckpointEnvelopeKey),
  DeleteCheckpoint(DeleteCheckpoint),
  BatchDeleteCheckpoints(BatchDeleteCheckpoints),

  // ==== SECRET ====
  CreateSecret(CreateSecret),
  UpdateSecret(UpdateSecret),
  UpdateSecretData(UpdateSecretData),
  UpdateSecretEncryptionKey(UpdateSecretEncryptionKey),
  RotateSecretEnvelopeKey(RotateSecretEnvelopeKey),
  DeleteSecret(DeleteSecret),
  BatchDeleteSecrets(BatchDeleteSecrets),

  // ==== ENCRYPTION KEY ====
  CreateEncryptionKey(CreateEncryptionKey),
  UpdateEncryptionKey(UpdateEncryptionKey),
  InitializeEncryptionKey(InitializeEncryptionKey),
  UninitializeEncryptionKey(UninitializeEncryptionKey),
  DeleteEncryptionKey(DeleteEncryptionKey),
  BatchDeleteEncryptionKeys(BatchDeleteEncryptionKeys),

  // ==== POLICY ====
  CreatePolicy(CreatePolicy),
  UpdatePolicy(UpdatePolicy),
  DeletePolicy(DeletePolicy),
  BatchDeletePolicies(BatchDeletePolicies),
}

pub fn router() -> Router {
  Router::new()
    .route("/", post(handler))
    .route("/{variant}", post(variant_handler))
    .layer(axum::middleware::from_fn(
      authenticate_request::<CicadaAuthImpl, true>,
    ))
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
  let res = tokio::spawn(task(request, client))
    .await
    .context("failure in spawned task");

  res?
}

/// Spawn a task to handle write requests
/// to ensure they finish even if client disconnects.
async fn task(
  request: WriteRequest,
  client: Client,
) -> mogh_error::Result<axum::response::Response> {
  let req_id = Uuid::new_v4();
  let method: WriteRequestMethod = (&request).into();

  info!("WRITE REQUEST {req_id} | METHOD: {method} | {client}",);

  let res = request.resolve(&WriteArgs { client }).await;

  if let Err(e) = &res {
    warn!(
      "WRITE REQUEST {req_id} | METHOD: {method} | ERROR: {:#}",
      e.error
    );
  }

  res.map(|res| res.0)
}
