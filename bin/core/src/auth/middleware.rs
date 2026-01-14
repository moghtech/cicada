use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context as _, anyhow};
use axum::{
  extract::{OriginalUri, Request},
  http::{HeaderMap, Method, StatusCode, Uri},
  middleware::Next,
  response::Response,
};
use cicada_client::{
  entities::{
    ClientType, device::DeviceRecord,
    onboarding_key::OnboardingKeyRecord, user::UserEntity,
  },
  pki_auth_prologue,
};
use futures_util::TryFutureExt as _;
use mogh_auth_server::request_ip::RequestIp;
use mogh_error::AddStatusCodeError as _;
use mogh_pki::{key::Pkcs8PrivateKey, one_way::OneWayNoiseHandshake};
use mogh_rate_limit::WithFailureRateLimit as _;

use crate::{
  auth::{GENERAL_RATE_LIMITER, JWT_PROVIDER},
  config::core_keys,
  db::query,
};

#[derive(Clone)]
pub enum Client {
  /// The user
  User(UserEntity),
  /// The device
  Device(DeviceRecord),
  /// The onboarding key
  OnboardingKey(OnboardingKeyRecord),
}

impl Client {
  pub fn sanitize(&mut self) {
    match self {
      // User entity already sanitized
      Client::User(_) => {}
      Client::Device(device_record) => device_record.sanitize(),
      Client::OnboardingKey(onboarding_key_record) => {
        onboarding_key_record.sanitize()
      }
    }
  }

  pub fn only_users(&self) -> mogh_error::Result<()> {
    if matches!(self, Client::User(_)) {
      Ok(())
    } else {
      Err(
        anyhow!("This method is only for user type clients")
          .status_code(StatusCode::UNAUTHORIZED),
      )
    }
  }

  pub fn not_onboarding_key(&self) -> mogh_error::Result<()> {
    if matches!(self, Client::OnboardingKey(_)) {
      Err(
        anyhow!("This method is not for onboarding type clients")
          .status_code(StatusCode::UNAUTHORIZED),
      )
    } else {
      Ok(())
    }
  }

  pub fn as_user(&self) -> mogh_error::Result<&UserEntity> {
    if let Client::User(user) = self {
      Ok(user)
    } else {
      Err(
        anyhow!("This method is only for user type clients")
          .status_code(StatusCode::UNAUTHORIZED),
      )
    }
  }
}

/// Middleware to authenticate incoming requests
/// using JWT or Api Key. It will attach the calling
/// client UserRecord to the request extensions.
pub async fn auth_request(
  RequestIp(ip): RequestIp,
  OriginalUri(uri): OriginalUri,
  mut req: Request,
  next: Next,
) -> mogh_error::Result<Response> {
  let mut client =
    get_client_from_request(req.method(), &uri, req.headers())
      .map_err(|e| e.status_code(StatusCode::UNAUTHORIZED))
      .with_failure_rate_limit_using_ip(&GENERAL_RATE_LIMITER, &ip)
      .await?;
  // Sanitize the user for safety before
  // attaching to the request handlers.
  client.sanitize();
  req.extensions_mut().insert(client);
  Ok(next.run(req).await)
}

pub async fn get_client_from_request(
  method: &Method,
  uri: &Uri,
  headers: &HeaderMap,
) -> anyhow::Result<Client> {
  match (
    headers.get("authorization"),
    headers.get("x-api-type"),
    headers.get("x-api-signature"),
    headers.get("x-api-timestamp"),
  ) {
    (Some(jwt), _, _, _) => {
      // USE JWT
      let jwt = jwt.to_str().context("JWT is not valid UTF-8")?;
      let user_id = JWT_PROVIDER.decode_sub(jwt)?;
      let user = query::user::get_user_entity(user_id).await?;
      if user.enabled {
        Ok(Client::User(user))
      } else {
        Err(anyhow!("Invalid client credentials"))
      }
    }
    (None, Some(client_type), Some(signature), Some(timestamp)) => {
      // USE API TYPE / SIGNATURE
      let client_type = client_type
        .to_str()
        .context("X-API-TYPE is not valid UTF-8")?
        .parse::<ClientType>()
        .context("X-API-TYPE is invalid")?;
      let signature = signature
        .to_str()
        .context("X-API-SIGNATURE is not valid UTF-8")?;
      // let signature = BASE
      let timestamp = timestamp
        .to_str()
        .context("X-API-TIMESTAMP is not valid UTF-8")?
        .parse::<i64>()?;

      let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_millis() as i64;

      // Ensure timestamp is ~now
      if (now - timestamp).abs() > 1_000 {
        return Err(anyhow!("Invalid client credentials"));
      }

      let prologue = pki_auth_prologue(method, uri, timestamp);

      let mut handshake = OneWayNoiseHandshake::new_responder(
        &Pkcs8PrivateKey::maybe_raw_bytes(
          core_keys().load().private.as_str(),
        )?,
        prologue.as_bytes(),
      )?;

      // Server now has client public key
      let public_key =
        handshake.validate_signature(signature)?.into_inner();

      match client_type {
        // Check against user api keys
        ClientType::User => {
          //
          todo!()
        }
        // Check against device public keys
        ClientType::Device => {
          let device =
            query::device::find_device_with_public_key(public_key)
              .await?
              .context("Invalid client credentials")?;
          if device.enabled {
            Ok(Client::Device(device))
          } else {
            Err(anyhow!("Invalid client credentials"))
          }
        }
        // Check against onboarding key public keys
        ClientType::OnboardingKey => {
          let onboarding_key =
            query::onboarding_key::find_onboarding_key_with_public_key(public_key)
              .await?
              .context("Invalid client credentials")?;
          if onboarding_key.enabled {
            Ok(Client::OnboardingKey(onboarding_key))
          } else {
            Err(anyhow!("Invalid client credentials"))
          }
        }
      }
    }
    _ => {
      // AUTH FAIL
      Err(anyhow!(
        "Must attach either AUTHORIZATION header with jwt OR headers X-API-TYPE and X-API-SIGNATURE"
      ))
    }
  }
}
