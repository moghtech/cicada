use axum::{Extension, Router, extract::Path, routing::post};
use cicada_client::api::read::{
  device::*, encryption_key::*, filesystem::*, node::*,
  onboarding_key::*, secret::*, *,
};
use mogh_auth_server::middleware::authenticate_request;
use mogh_error::{Json, Response};
use mogh_resolver::Resolve;
use serde::{Deserialize, Serialize};
use serde_json::json;
use strum::{Display, EnumDiscriminants};
use surrealdb::types::Uuid;
use typeshare::typeshare;

use crate::auth::CicadaAuthImpl;
use crate::{
  api::Variant, auth::middleware::Client, db::query::user::get_user,
};

pub mod device;
pub mod encryption_key;
pub mod filesystem;
pub mod node;
pub mod onboarding_key;
pub mod secret;

pub struct ReadArgs {
  client: Client,
}

#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, Resolve, EnumDiscriminants,
)]
#[strum_discriminants(name(ReadRequestMethod), derive(Display))]
#[args(ReadArgs)]
#[response(Response)]
#[error(mogh_error::Error)]
#[serde(tag = "type", content = "params")]
enum ReadRequest {
  GetVersion(GetVersion),
  GetUsername(GetUsername),

  // ==== DEVICE ====
  ListDevices(ListDevices),
  GetDevice(GetDevice),

  // ==== ONBOARDING KEY ====
  ListOnboardingKeys(ListOnboardingKeys),
  GetOnboardingKey(GetOnboardingKey),

  // ==== FILESYSTEM ====
  ListFilesystems(ListFilesystems),
  GetFilesystem(GetFilesystem),

  // ==== NODE ====
  ListNodes(ListNodes),
  GetNode(GetNode),
  FindNode(FindNode),
  FindNodeWithPath(FindNodeWithPath),

  // ==== SECRET ====
  ListSecrets(ListSecrets),
  GetSecret(GetSecret),
  FindSecret(FindSecret),

  // ==== ENCRYPTION KEY ====
  ListEncryptionKeys(ListEncryptionKeys),
  GetEncryptionKey(GetEncryptionKey),
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
  let req: ReadRequest = serde_json::from_value(json!({
    "type": variant,
    "params": params,
  }))?;
  handler(client, Json(req)).await
}

async fn handler(
  Extension(client): Extension<Client>,
  Json(request): Json<ReadRequest>,
) -> mogh_error::Result<axum::response::Response> {
  // Onboarding keys can't be used to directly access read api.
  client.not_onboarding_key()?;

  let req_id = Uuid::new_v4();
  let method: ReadRequestMethod = (&request).into();

  debug!("READ REQUEST {req_id} | METHOD: {method} | {client}");

  let res = request.resolve(&ReadArgs { client }).await;

  if let Err(e) = &res {
    debug!(
      "READ REQUEST {req_id} | METHOD: {method} | ERROR: {:#}",
      e.error
    );
  }

  res.map(|res| res.0)
}

//

impl Resolve<ReadArgs> for GetVersion {
  async fn resolve(
    self,
    ReadArgs { client: _client }: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    Ok(GetVersionResponse {
      version: env!("CARGO_PKG_VERSION").to_string(),
    })
  }
}

//

impl Resolve<ReadArgs> for GetUsername {
  async fn resolve(
    self,
    ReadArgs { .. }: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    let username = get_user(&self.user_id).await?.username;
    Ok(GetUsernameResponse {
      username,
      avatar: None,
    })
  }
}
