use std::sync::{Arc, LazyLock};

use anyhow::anyhow;
use async_timing_util::{Timelength, get_timelength_in_ms};
use axum::http::StatusCode;
use cicada_client::entities::user::UserRecord;
use mogh_auth_client::passkey::Passkey;
// use cicada_client::entities::user::u
use mogh_auth_server::{
  AuthImpl,
  args::RequestClientArgs,
  provider::{jwt::JwtProvider, passkey::PasskeyProvider},
  rand::random_string,
  user::{AuthUserImpl, BoxAuthUser},
};
use mogh_error::AddStatusCode;
use mogh_rate_limit::RateLimiter;

use crate::{
  config::core_config,
  db::query::user::{
    find_user_with_username, get_user, sign_up_local_user,
  },
};

pub mod middleware;

static JWT_PROVIDER: LazyLock<JwtProvider> = LazyLock::new(|| {
  let config = core_config();
  let secret = if config.jwt_secret.is_empty() {
    random_string(40)
  } else {
    config.jwt_secret.clone()
  };
  JwtProvider::new(
    secret.as_bytes(),
    get_timelength_in_ms(
      config.jwt_ttl.to_string().parse().unwrap_or_else(|e| {
        warn!(
          "Failed to parse 'jwt_ttl' | Using default of 1-day | {e:?}"
        );
        Timelength::OneDay
      }),
    ),
  )
});

static GENERAL_RATE_LIMITER: LazyLock<Arc<RateLimiter>> =
  LazyLock::new(|| {
    let config = core_config();
    RateLimiter::new(
      config.auth_rate_limit_disabled,
      config.auth_rate_limit_max_attempts as usize,
      config.auth_rate_limit_window_seconds,
    )
  });

static LOCAL_LOGIN_RATE_LIMITER: LazyLock<Arc<RateLimiter>> =
  LazyLock::new(|| {
    let config = core_config();
    RateLimiter::new(
      config.auth_rate_limit_disabled,
      config.auth_rate_limit_max_attempts as usize,
      config.auth_rate_limit_window_seconds,
    )
  });

pub struct AuthUser(UserRecord);

impl AuthUserImpl for AuthUser {
  fn id(&self) -> &str {
    &self.0.id.0
  }

  fn username(&self) -> &str {
    &self.0.name
  }

  fn hashed_password(&self) -> Option<&str> {
    if self.0.password.is_empty() {
      None
    } else {
      Some(&self.0.password)
    }
  }
}

pub struct CicadaAuthImpl {
  client: RequestClientArgs,
}

impl AuthImpl for CicadaAuthImpl {
  fn from_client(client: RequestClientArgs) -> Self
  where
    Self: Sized,
  {
    Self { client }
  }

  fn client(&self) -> &RequestClientArgs {
    &self.client
  }

  fn app_name(&self) -> &str {
    "Cicada"
  }

  fn get_user(
    &self,
    user_id: String,
  ) -> mogh_auth_server::DynFuture<mogh_error::Result<BoxAuthUser>>
  {
    Box::pin(async move {
      Ok(Box::new(AuthUser(get_user(&user_id).await?)) as BoxAuthUser)
    })
  }

  // =========
  // = STATE =
  // =========

  fn jwt_provider(&self) -> &JwtProvider {
    &JWT_PROVIDER
  }

  fn passkey_provider(&self) -> Option<&PasskeyProvider> {
    static PASSKEY_PROVIDER: LazyLock<Option<PasskeyProvider>> =
      LazyLock::new(|| {
        PasskeyProvider::new(&core_config().host)
          .inspect_err(|e| {
            warn!("Invalid 'host' for passkey provider | {e:#}")
          })
          .ok()
      });
    PASSKEY_PROVIDER.as_ref()
  }

  fn general_rate_limiter(&self) -> &RateLimiter {
    &GENERAL_RATE_LIMITER
  }

  // ==============
  // = LOCAL AUTH =
  // ==============

  fn local_login_rate_limiter(&self) -> &RateLimiter {
    &LOCAL_LOGIN_RATE_LIMITER
  }

  fn sign_up_local_user(
    &self,
    username: String,
    hashed_password: String,
    _no_users_exist: bool,
  ) -> mogh_auth_server::DynFuture<mogh_error::Result<String>> {
    Box::pin(async move {
      sign_up_local_user(username, hashed_password, true)
        .await
        .map_err(Into::into)
    })
  }

  fn find_user_with_username(
    &self,
    username: String,
  ) -> mogh_auth_server::DynFuture<mogh_error::Result<BoxAuthUser>>
  {
    Box::pin(async move {
      let user = find_user_with_username(username)
        .await
        .map_err(|_| anyhow!("Invalid login credentials"))
        .status_code(StatusCode::UNAUTHORIZED)?;
      Ok(Box::new(AuthUser(user)) as BoxAuthUser)
    })
  }

  fn update_user_username(
    &self,
    user_id: String,
    username: String,
  ) -> mogh_auth_server::DynFuture<mogh_error::Result<()>> {
    Box::pin(async { todo!() })
  }

  fn update_user_password(
    &self,
    user_id: String,
    password: String,
  ) -> mogh_auth_server::DynFuture<mogh_error::Result<()>> {
    Box::pin(async { todo!() })
  }

  // ===============
  // = PASSKEY 2FA =
  // ===============

  fn update_user_stored_passkey(
    &self,
    user_id: String,
    passkey: Option<Passkey>,
  ) -> mogh_auth_server::DynFuture<mogh_error::Result<()>> {
    Box::pin(async { todo!() })
  }

  // ===============
  // = TOTP 2FA =
  // ===============

  fn update_user_stored_totp(
    &self,
    user_id: String,
    encoded_secret: String,
    hashed_recovery_codes: Vec<String>,
  ) -> mogh_auth_server::DynFuture<mogh_error::Result<()>> {
    Box::pin(async { todo!() })
  }

  fn remove_user_stored_totp(
    &self,
    user_id: String,
  ) -> mogh_auth_server::DynFuture<mogh_error::Result<()>> {
    Box::pin(async { todo!() })
  }
}
