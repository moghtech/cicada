use std::time::Instant;

use axum::{Extension, Router, extract::Path, routing::post};
use cicada_client::{
  api::read::{
    GetUser, GetUsername, GetUsernameResponse, GetVersion,
    GetVersionResponse,
    filesystem::ListFilesystems,
    node::{FindNode, GetNode, ListNodes},
  },
  entities::user::UserRecord,
};
use mogh_error::{Json, Response};
use resolver_api::Resolve;
use serde::{Deserialize, Serialize};
use serde_json::json;
use surrealdb::types::Uuid;
use typeshare::typeshare;

use crate::{
  api::Variant, auth::middleware::auth_request,
  db::query::user::get_user,
};

pub mod filesystem;
pub mod node;

pub struct ReadArgs {
  user: UserRecord,
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
  user: Extension<UserRecord>,
  Path(Variant { variant }): Path<Variant>,
  Json(params): Json<serde_json::Value>,
) -> mogh_error::Result<axum::response::Response> {
  let req: ReadRequest = serde_json::from_value(json!({
    "type": variant,
    "params": params,
  }))?;
  handler(user, Json(req)).await
}

async fn handler(
  Extension(user): Extension<UserRecord>,
  Json(request): Json<ReadRequest>,
) -> mogh_error::Result<axum::response::Response> {
  let timer = Instant::now();
  let req_id = Uuid::new_v4();
  // debug!("/read request | user: {}", user.username);
  let res = request.resolve(&ReadArgs { user }).await;
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
    ReadArgs { user }: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    Ok(user.clone())
  }
}

//

impl Resolve<ReadArgs> for GetUsername {
  async fn resolve(
    self,
    ReadArgs { .. }: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    let username = get_user(&self.user_id).await?.name;
    Ok(GetUsernameResponse {
      username,
      avatar: None,
    })
  }
}
