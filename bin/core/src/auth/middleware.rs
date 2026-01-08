use anyhow::{Context as _, anyhow};
use axum::{
  extract::Request,
  http::{HeaderMap, StatusCode},
  middleware::Next,
  response::Response,
};
use cicada_client::entities::user::UserRecord;
use futures_util::TryFutureExt as _;
use mogh_auth_server::request_ip::RequestIp;
use mogh_error::AddStatusCodeError as _;
use mogh_rate_limit::WithFailureRateLimit as _;

use crate::{
  auth::{GENERAL_RATE_LIMITER, JWT_PROVIDER},
  db::query::user::get_user,
};

/// Middleware to authenticate incoming requests
/// using JWT or Api Key. It will attach the calling
/// client UserRecord to the request extensions.
pub async fn auth_request(
  RequestIp(ip): RequestIp,
  mut req: Request,
  next: Next,
) -> mogh_error::Result<Response> {
  let mut user = authenticate_check_enabled(req.headers())
    .map_err(|e| e.status_code(StatusCode::UNAUTHORIZED))
    .with_failure_rate_limit_using_ip(&GENERAL_RATE_LIMITER, &ip)
    .await?;
  // Sanitize the user for safety before
  // attaching to the request handlers.
  user.sanitize();
  req.extensions_mut().insert(user);
  Ok(next.run(req).await)
}

pub async fn authenticate_check_enabled(
  headers: &HeaderMap,
) -> anyhow::Result<UserRecord> {
  let user_id = get_user_id_from_headers(headers).await?;
  let user = get_user(&user_id)
    .await
    .map_err(|_| anyhow!("Invalid user credentials"))?;
  if user.enabled {
    Ok(user)
  } else {
    Err(anyhow!("Invalid user credentials"))
  }
}

pub async fn get_user_id_from_headers(
  headers: &HeaderMap,
) -> anyhow::Result<String> {
  match (
    headers.get("authorization"),
    headers.get("x-api-key"),
    headers.get("x-api-secret"),
  ) {
    (Some(jwt), _, _) => {
      // USE JWT
      let jwt = jwt.to_str().context("JWT is not valid UTF-8")?;
      JWT_PROVIDER.decode_sub(jwt)
    }
    (None, Some(key), Some(secret)) => {
      // USE API KEY / SECRET
      let key =
        key.to_str().context("X-API-KEY is not valid UTF-8")?;
      let secret =
        secret.to_str().context("X-API-SECRET is not valid UTF-8")?;
      todo!()
    }
    _ => {
      // AUTH FAIL
      Err(anyhow!(
        "Must attach either AUTHORIZATION header with jwt OR pass X-API-KEY and X-API-SECRET"
      ))
    }
  }
}
