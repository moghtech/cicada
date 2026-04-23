use axum::http::{HeaderMap, StatusCode};
use cicada_client::entities::{
  ClientType, Iso8601Timestamp, device::DeviceRecord,
  onboarding_key::OnboardingKeyRecord, user::UserEntity,
};
use mogh_auth_server::RequestAuthentication;
use mogh_error::{
  AddStatusCodeError as _,
  anyhow::{Context as _, anyhow},
};

use crate::db::query::{self, api_key::find_api_key};

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

  pub fn is_admin_user(&self) -> bool {
    if let Client::User(user) = self {
      user.admin
    } else {
      false
    }
  }

  pub fn not_device(&self) -> mogh_error::Result<()> {
    if matches!(self, Client::Device(_)) {
      Err(
        anyhow!("This method is not for device type clients")
          .status_code(StatusCode::UNAUTHORIZED),
      )
    } else {
      Ok(())
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

  pub fn into_user(self) -> mogh_error::Result<UserEntity> {
    if let Client::User(user) = self {
      Ok(user)
    } else {
      Err(
        anyhow!("This method is only for user type clients")
          .status_code(StatusCode::UNAUTHORIZED),
      )
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

  /// When client is user, they must be admin.
  /// Other client types are still allowed.
  pub fn only_admin_users(&self) -> mogh_error::Result<()> {
    if let Client::User(user) = self
      && !user.admin
    {
      Err(
        anyhow!("This method is only for admin users")
          .status_code(StatusCode::UNAUTHORIZED),
      )
    } else {
      Ok(())
    }
  }

  /// This ONLY allows user admin clients.
  /// Other client types are NOT allowed.
  pub fn admin_only(&self) -> mogh_error::Result<()> {
    if let Client::User(user) = self
      && user.admin
    {
      Ok(())
    } else {
      Err(
        anyhow!("This method is admin-only")
          .status_code(StatusCode::UNAUTHORIZED),
      )
    }
  }
}

impl std::fmt::Display for Client {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Client::User(client) => f.write_fmt(format_args!(
        "USER: {} ({})",
        client.username, client.id.0
      )),
      Client::Device(client) => f.write_fmt(format_args!(
        "DEVICE: {} ({})",
        client.name, client.id.0
      )),
      Client::OnboardingKey(client) => f.write_fmt(format_args!(
        "ONBOARDING KEY: {} ({})",
        client.name, client.id.0
      )),
    }
  }
}

pub async fn get_client_from_auth(
  auth: RequestAuthentication,
  require_user_enabled: bool,
  headers: &HeaderMap,
) -> mogh_error::Result<Client> {
  match auth {
    RequestAuthentication::UserId(user_id) => {
      let user = query::user::get_user_entity(user_id).await?;
      if !require_user_enabled || user.enabled {
        Ok(Client::User(user))
      } else {
        Err(
          anyhow!("Invalid client credentials")
            .status_code(StatusCode::UNAUTHORIZED),
        )
      }
    }
    RequestAuthentication::KeyAndSecret { key, secret } => {
      let key = find_api_key(key).await?;

      if !key.enabled {
        return Err(
          anyhow!("Invalid client credentials")
            .status_code(StatusCode::UNAUTHORIZED),
        );
      }

      if let Some(expires) = key.expires
        && expires < Iso8601Timestamp::now()
      {
        return Err(
          anyhow!("Invalid client credentials")
            .status_code(StatusCode::UNAUTHORIZED),
        );
      }

      let Some(secret_hash) = key.secret else {
        return Err(
          anyhow!("Invalid client credentials")
            .status_code(StatusCode::UNAUTHORIZED),
        );
      };

      if !bcrypt::verify(secret, &secret_hash).map_err(|_| {
        anyhow!("Invalid user credentials")
          .status_code(StatusCode::UNAUTHORIZED)
      })? {
        // secret mismatch
        return Err(
          anyhow!("Invalid user credentials")
            .status_code(StatusCode::UNAUTHORIZED),
        );
      }

      let user = query::user::get_user_entity(key.user.0).await?;
      if !require_user_enabled || user.enabled {
        Ok(Client::User(user))
      } else {
        Err(
          anyhow!("Invalid client credentials")
            .status_code(StatusCode::UNAUTHORIZED),
        )
      }
    }
    RequestAuthentication::PublicKey(public_key) => {
      let client_type = headers
        .get("x-api-type")
        .context(
          "Missing X-API-TYPE: User, Device, or OnboardingKey",
        )?
        .to_str()
        .context("X-API-TYPE is not valid UTF-8")?
        .parse::<ClientType>()
        .context("X-API-TYPE is invalid")?;
      match client_type {
        ClientType::User => {
          Err(anyhow!("Not implemented").into())
        }
        ClientType::Device => {
          let device =
            query::device::find_device_with_public_key(public_key)
              .await?
              .context("Invalid client credentials")?;
          if device.enabled {
            Ok(Client::Device(device))
          } else {
            Err(
              anyhow!("Invalid client credentials")
                .status_code(StatusCode::UNAUTHORIZED),
            )
          }
        }
        ClientType::OnboardingKey => {
          let onboarding_key =
            query::onboarding_key::find_onboarding_key_with_public_key(public_key)
              .await?
              .context("Invalid client credentials")?;

          if !onboarding_key.enabled {
            return Err(
              anyhow!("Invalid client credentials")
                .status_code(StatusCode::UNAUTHORIZED),
            );
          }

          if let Some(expires) = onboarding_key.expires
            && expires < Iso8601Timestamp::now()
          {
            return Err(
              anyhow!("Invalid client credentials")
                .status_code(StatusCode::UNAUTHORIZED),
            );
          }

          Ok(Client::OnboardingKey(onboarding_key))
        }
      }
    }
  }
}
