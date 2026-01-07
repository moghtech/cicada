use std::sync::{Arc, LazyLock};

use anyhow::anyhow;
use axum::http::StatusCode;
use cicada_client::entities::user::UserRecord;
use mogh_auth_client::passkey::Passkey;
// use cicada_client::entities::user::u
use mogh_auth_server::{
  AuthImpl,
  args::RequestClientArgs,
  provider::{jwt::JwtProvider, passkey::PasskeyProvider},
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

static JWT_PROVIDER: LazyLock<JwtProvider> =
  LazyLock::new(|| JwtProvider::new(&[], 600_000));
static GENERAL_RATE_LIMITER: LazyLock<Arc<RateLimiter>> =
  LazyLock::new(|| RateLimiter::new(true, 0, 0));

pub struct AuthUser(UserRecord);

impl AuthUserImpl for AuthUser {
  fn id(&self) -> &str {
    &self.0.id.0
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
    user_id: &str,
  ) -> mogh_auth_server::DynFuture<mogh_error::Result<BoxAuthUser>>
  {
    let user_id = user_id.to_string();
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
    username: &str,
  ) -> mogh_auth_server::DynFuture<mogh_error::Result<BoxAuthUser>>
  {
    let username = username.to_string();
    Box::pin(async move {
      let user = find_user_with_username(username)
        .await
        .map_err(|_| anyhow!("Invalid login credentials"))
        .status_code(StatusCode::UNAUTHORIZED)?;
      Ok(Box::new(AuthUser(user)) as BoxAuthUser)
    })
  }

  fn update_user_stored_passkey(
    &self,
    user_id: &str,
    passkey: Passkey,
  ) -> mogh_auth_server::DynFuture<mogh_error::Result<()>> {
    Box::pin(async { todo!() })
  }
}
