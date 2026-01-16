use std::time::Instant;

use anyhow::anyhow;
use axum::{
  Extension, Router, extract::Path, http::StatusCode, routing::post,
};
use cicada_client::api::read::{
  device::*, encryption_key::*, filesystem::*, node::*,
  onboarding_key::*, *,
};
use mogh_error::{AddStatusCodeError, Json, Response};
use resolver_api::Resolve;
use serde::{Deserialize, Serialize};
use serde_json::json;
use surrealdb::types::Uuid;
use typeshare::typeshare;

use crate::{
  api::Variant,
  auth::middleware::{Client, auth_request},
  db::query::user::get_user,
};

pub mod device;
pub mod encryption_key;
pub mod filesystem;
pub mod node;
pub mod onboarding_key;

pub struct ReadArgs {
  client: Client,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
#[args(ReadArgs)]
#[response(Response)]
#[error(mogh_error::Error)]
#[serde(tag = "type", content = "params")]
enum ReadRequest {
  GetVersion(GetVersion),
  GetUser(GetUser),
  GetUsername(GetUsername),

  // ==== DEVICE ====
  ListDevices(ListDevices),
  GetDevice(GetDevice),

  // ==== ONBOARDING KEY ====
  ListOnboardingKeys(ListOnboardingKeys),
  GetOnboardingKey(GetOnboardingKey),

  // ==== FILESYSTEM ====
  ListFilesystems(ListFilesystems),

  // ==== NODE ====
  ListNodes(ListNodes),
  GetNode(GetNode),
  FindNode(FindNode),

  // ==== ENCRYPTION KEY ====
  ListEncryptionKeys(ListEncryptionKeys),
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
  let timer = Instant::now();
  let req_id = Uuid::new_v4();
  // debug!("/read request | user: {}", user.username);
  let res = request.resolve(&ReadArgs { client }).await;
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

//

impl Resolve<ReadArgs> for GetUser {
  async fn resolve(
    self,
    ReadArgs { client }: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    let Client::User(user) = client else {
      return Err(
        anyhow!("Client is not user")
          .status_code(StatusCode::BAD_REQUEST),
      );
    };
    Ok(user.clone())
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
