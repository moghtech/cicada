use std::sync::{Arc, LazyLock};

use async_timing_util::{Timelength, get_timelength_in_ms};
use cicada_client::entities::user::UserRecord;
use mogh_auth_client::{
  api::login::LoginProvider,
  config::{NamedOauthConfig, OidcConfig},
  passkey::Passkey,
};
// use cicada_client::entities::user::u
use mogh_auth_server::{
  AuthImpl, RequestClientArgs,
  provider::{
    jwt::JwtProvider, oidc::SubjectIdentifier,
    passkey::PasskeyProvider,
  },
  rand::random_string,
  user::{AuthUserImpl, BoxAuthUser},
};
use mogh_rate_limit::RateLimiter;

use crate::{config::core_config, db::query::user::*};

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

  fn passkey(&self) -> Option<Passkey> {
    let passkey = self.0.passkey.as_ref()?;
    serde_json::from_str(&serde_json::to_string(passkey).ok()?)
      .inspect_err(|e| {
        warn!(
          "User {} ({}) | Invalid passkey on database | {e:?}",
          self.username(),
          self.id(),
        )
      })
      .ok()
  }

  fn totp_secret(&self) -> Option<&str> {
    if self.0.totp_secret.is_empty() {
      None
    } else {
      Some(&self.0.totp_secret)
    }
  }

  fn external_skip_2fa(&self) -> bool {
    self.0.external_skip_2fa
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

  fn app_name(&self) -> &'static str {
    "Cicada"
  }

  fn host(&self) -> &str {
    static AUTH_HOST: LazyLock<String> =
      LazyLock::new(|| format!("{}/auth", core_config().host));
    &AUTH_HOST
  }

  fn post_link_redirect(&self) -> &str {
    static POST_LINK_REDIRECT: LazyLock<String> =
      LazyLock::new(|| format!("{}/profile", core_config().host));
    &POST_LINK_REDIRECT
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

  fn no_users_exist(
    &self,
  ) -> mogh_auth_server::DynFuture<mogh_error::Result<bool>> {
    Box::pin(async { no_users_exist().await.map_err(Into::into) })
  }

  fn locked_usernames(&self) -> &'static [String] {
    &core_config().lock_login_credentials_for
  }

  fn registration_disabled(&self) -> bool {
    core_config().disable_user_registration
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

  fn local_auth_enabled(&self) -> bool {
    core_config().local_auth
  }

  fn local_login_rate_limiter(&self) -> &RateLimiter {
    &LOCAL_LOGIN_RATE_LIMITER
  }

  fn sign_up_local_user(
    &self,
    username: String,
    hashed_password: String,
    no_users_exist: bool,
  ) -> mogh_auth_server::DynFuture<mogh_error::Result<String>> {
    Box::pin(async move {
      sign_up_local_user(
        username,
        hashed_password,
        no_users_exist || core_config().enable_new_users,
      )
      .await
      .map_err(Into::into)
    })
  }

  fn find_user_with_username(
    &self,
    username: String,
  ) -> mogh_auth_server::DynFuture<
    mogh_error::Result<Option<BoxAuthUser>>,
  > {
    Box::pin(async move {
      let user = find_user_with_username(username)
        .await?
        .map(|user| Box::new(AuthUser(user)) as BoxAuthUser);
      Ok(user)
    })
  }

  fn update_user_username(
    &self,
    user_id: String,
    username: String,
  ) -> mogh_auth_server::DynFuture<mogh_error::Result<()>> {
    Box::pin(async {
      update_user_fields(
        user_id,
        UpdateUser {
          name: Some(username),
          ..Default::default()
        },
      )
      .await
      .map(|_| ())
      .map_err(Into::into)
    })
  }

  fn update_user_password(
    &self,
    user_id: String,
    password: String,
  ) -> mogh_auth_server::DynFuture<mogh_error::Result<()>> {
    Box::pin(async {
      update_user_fields(
        user_id,
        UpdateUser {
          password: Some(password),
          ..Default::default()
        },
      )
      .await
      .map(|_| ())
      .map_err(Into::into)
    })
  }

  // =============
  // = OIDC AUTH =
  // =============

  fn oidc_config(&self) -> &OidcConfig {
    &core_config().oidc
  }

  fn find_user_with_oidc_subject(
    &self,
    subject: SubjectIdentifier,
  ) -> mogh_auth_server::DynFuture<
    mogh_error::Result<Option<BoxAuthUser>>,
  > {
    Box::pin(async move {
      let user = find_user_with_oidc_subject(subject.into())
        .await?
        .map(|user| Box::new(AuthUser(user)) as BoxAuthUser);
      Ok(user)
    })
  }

  fn sign_up_oidc_user(
    &self,
    username: String,
    subject: SubjectIdentifier,
    no_users_exist: bool,
  ) -> mogh_auth_server::DynFuture<mogh_error::Result<String>> {
    Box::pin(async move {
      sign_up_oidc_user(
        username,
        subject.into(),
        no_users_exist || core_config().enable_new_users,
      )
      .await
      .map_err(Into::into)
    })
  }

  fn link_oidc_login(
    &self,
    user_id: String,
    subject: SubjectIdentifier,
  ) -> mogh_auth_server::DynFuture<mogh_error::Result<()>> {
    Box::pin(async move {
      update_user_fields(
        user_id,
        UpdateUser {
          oidc_subject: Some(subject.into()),
          ..Default::default()
        },
      )
      .await
      .map(|_| ())
      .map_err(Into::into)
    })
  }

  // ===============
  // = GITHUB AUTH =
  // ===============

  fn github_config(&self) -> &NamedOauthConfig {
    &core_config().github_oauth
  }

  fn find_user_with_github_id(
    &self,
    github_id: String,
  ) -> mogh_auth_server::DynFuture<
    mogh_error::Result<Option<BoxAuthUser>>,
  > {
    Box::pin(async move {
      let user = find_user_with_github_id(github_id)
        .await?
        .map(|user| Box::new(AuthUser(user)) as BoxAuthUser);
      Ok(user)
    })
  }

  fn link_github_login(
    &self,
    user_id: String,
    github_id: String,
  ) -> mogh_auth_server::DynFuture<mogh_error::Result<()>> {
    Box::pin(async move {
      update_user_fields(
        user_id,
        UpdateUser {
          github_id: Some(github_id),
          ..Default::default()
        },
      )
      .await
      .map(|_| ())
      .map_err(Into::into)
    })
  }

  fn sign_up_github_user(
    &self,
    username: String,
    github_id: String,
    no_users_exist: bool,
  ) -> mogh_auth_server::DynFuture<mogh_error::Result<String>> {
    Box::pin(async move {
      sign_up_github_user(
        username,
        github_id,
        no_users_exist || core_config().enable_new_users,
      )
      .await
      .map_err(Into::into)
    })
  }

  // ===============
  // = GOOGLE AUTH =
  // ===============

  fn google_config(&self) -> &NamedOauthConfig {
    &core_config().google_oauth
  }

  fn find_user_with_google_id(
    &self,
    google_id: String,
  ) -> mogh_auth_server::DynFuture<
    mogh_error::Result<Option<BoxAuthUser>>,
  > {
    Box::pin(async move {
      let user = find_user_with_google_id(google_id)
        .await?
        .map(|user| Box::new(AuthUser(user)) as BoxAuthUser);
      Ok(user)
    })
  }

  fn link_google_login(
    &self,
    user_id: String,
    google_id: String,
  ) -> mogh_auth_server::DynFuture<mogh_error::Result<()>> {
    Box::pin(async move {
      update_user_fields(
        user_id,
        UpdateUser {
          google_id: Some(google_id),
          ..Default::default()
        },
      )
      .await
      .map(|_| ())
      .map_err(Into::into)
    })
  }

  fn sign_up_google_user(
    &self,
    username: String,
    google_id: String,
    no_users_exist: bool,
  ) -> mogh_auth_server::DynFuture<mogh_error::Result<String>> {
    Box::pin(async move {
      sign_up_google_user(
        username,
        google_id,
        no_users_exist || core_config().enable_new_users,
      )
      .await
      .map_err(Into::into)
    })
  }

  // ==========
  // = UNLINK =
  // ==========

  fn unlink_login(
    &self,
    user_id: String,
    provider: LoginProvider,
  ) -> mogh_auth_server::DynFuture<mogh_error::Result<()>> {
    Box::pin(async move {
      let update = match provider {
        LoginProvider::Local => UpdateUser {
          password: Some(String::new()),
          ..Default::default()
        },
        LoginProvider::Oidc => UpdateUser {
          oidc_subject: Some(String::new()),
          ..Default::default()
        },
        LoginProvider::Github => UpdateUser {
          oidc_subject: Some(String::new()),
          ..Default::default()
        },
        LoginProvider::Google => UpdateUser {
          oidc_subject: Some(String::new()),
          ..Default::default()
        },
      };
      update_user_fields(user_id, update)
        .await
        .map(|_| ())
        .map_err(Into::into)
    })
  }

  // ===============
  // = PASSKEY 2FA =
  // ===============

  fn update_user_stored_passkey(
    &self,
    user_id: String,
    passkey: Option<Passkey>,
  ) -> mogh_auth_server::DynFuture<mogh_error::Result<()>> {
    Box::pin(async {
      update_user_passkey(user_id, passkey)
        .await
        .map(|_| ())
        .map_err(Into::into)
    })
  }

  // ============
  // = TOTP 2FA =
  // ============

  fn update_user_stored_totp(
    &self,
    user_id: String,
    totp_secret: String,
    _hashed_recovery_codes: Vec<String>,
  ) -> mogh_auth_server::DynFuture<mogh_error::Result<()>> {
    Box::pin(async {
      update_user_fields(
        user_id,
        UpdateUser {
          totp_secret: Some(totp_secret),
          ..Default::default()
        },
      )
      .await
      .map(|_| ())
      .map_err(Into::into)
    })
  }

  fn remove_user_stored_totp(
    &self,
    user_id: String,
  ) -> mogh_auth_server::DynFuture<mogh_error::Result<()>> {
    Box::pin(async {
      update_user_fields(
        user_id,
        UpdateUser {
          totp_secret: Some(String::new()),
          ..Default::default()
        },
      )
      .await
      .map(|_| ())
      .map_err(Into::into)
    })
  }

  // ============
  // = SKIP 2FA =
  // ============
  fn update_user_external_skip_2fa(
    &self,
    user_id: String,
    external_skip_2fa: bool,
  ) -> mogh_auth_server::DynFuture<mogh_error::Result<()>> {
    Box::pin(async move {
      update_user_fields(
        user_id,
        UpdateUser {
          external_skip_2fa: Some(external_skip_2fa),
          ..Default::default()
        },
      )
      .await
      .map(|_| ())
      .map_err(Into::into)
    })
  }
}
