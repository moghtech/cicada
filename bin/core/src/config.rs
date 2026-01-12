use std::{path::PathBuf, sync::OnceLock};

use anyhow::Context as _;
use cicada_client::entities::config::{
  core::{CoreConfig, DatabaseConfig, Env},
  logger::LogConfig,
};
use colored::Colorize as _;
use mogh_auth_client::config::{NamedOauthConfig, OidcConfig};
use mogh_config::ConfigLoader;
use mogh_pki::key::RotatableKeyPair;
use mogh_secret_file::maybe_read_item_from_file;

/// Should call in startup to ensure Core errors without valid private key.
pub fn core_keys() -> &'static RotatableKeyPair {
  static CORE_KEYS: OnceLock<RotatableKeyPair> = OnceLock::new();
  CORE_KEYS.get_or_init(|| {
    RotatableKeyPair::from_private_key_spec(
      mogh_pki::PkiType::OneWay,
      &core_config().private_key,
    )
    .unwrap()
  })
}

pub fn core_config() -> &'static CoreConfig {
  static CORE_CONFIG: OnceLock<CoreConfig> = OnceLock::new();
  CORE_CONFIG.get_or_init(|| {
    let env: Env = match envy::from_env()
      .context("Failed to parse Komodo Core environment")
    {
      Ok(env) => env,
      Err(e) => {
        panic!("{e:?}");
      }
    };

    let config = if env.cicada_config_paths.is_empty() {
      println!(
        "{}: No config paths found, using default config",
        "INFO".green(),
      );
      CoreConfig::default()
    } else {
      let config_keywords = env
        .cicada_config_keywords
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();

      println!(
        "{}: {}: {config_keywords:?}",
        "INFO".green(),
        "Config File Keywords".dimmed(),
      );

      (ConfigLoader {
        paths: &env
          .cicada_config_paths
          .iter()
          .map(PathBuf::as_path)
          .collect::<Vec<_>>(),
        match_wildcards: &config_keywords,
        include_file_name: ".ccoreinclude",
        merge_nested: env.cicada_merge_nested_config,
        extend_array: env.cicada_extend_config_arrays,
        debug_print: env.cicada_config_debug,
      })
      .load::<CoreConfig>()
      .expect("Failed at parsing config from paths")
    };

    // recreating CoreConfig here makes sure apply all env overrides applied.
    CoreConfig {
      title: env.cicada_title.unwrap_or(config.title),
      host: env.cicada_host.unwrap_or(config.host),
      port: env.cicada_port.unwrap_or(config.port),
      bind_ip: env.cicada_bind_ip.unwrap_or(config.bind_ip),
      jwt_secret: maybe_read_item_from_file(
        env.cicada_jwt_secret_file,
        env.cicada_jwt_secret,
      )
      .unwrap_or(config.jwt_secret),
      jwt_ttl: env.cicada_jwt_ttl.unwrap_or(config.jwt_ttl),
      private_key: maybe_read_item_from_file(
        env.cicada_private_key_file,
        env.cicada_private_key,
      )
      .unwrap_or(config.private_key),
      database: DatabaseConfig {
        uri: env.cicada_database_uri.unwrap_or(config.database.uri),
        username: maybe_read_item_from_file(
          env.cicada_database_username_file,
          env.cicada_database_username,
        )
        .unwrap_or(config.database.username),
        password: maybe_read_item_from_file(
          env.cicada_database_password_file,
          env.cicada_database_password,
        )
        .unwrap_or(config.database.password),
        namespace: env
          .cicada_database_namespace
          .unwrap_or(config.database.namespace),
        db_name: env
          .cicada_database_db_name
          .unwrap_or(config.database.db_name),
      },
      local_auth: env.cicada_local_auth.unwrap_or(config.local_auth),
      disable_user_registration: env
        .cicada_disable_user_registration
        .unwrap_or(config.disable_user_registration),
      enable_new_users: env
        .cicada_enable_new_users
        .unwrap_or(config.enable_new_users),
      lock_login_credentials_for: env
        .cicada_lock_login_credentials_for
        .unwrap_or(config.lock_login_credentials_for),
      oidc: OidcConfig {
        enabled: env
          .cicada_oidc_enabled
          .unwrap_or(config.oidc.enabled),
        provider: env
          .cicada_oidc_provider
          .unwrap_or(config.oidc.provider),
        redirect_host: env
          .cicada_oidc_redirect_host
          .unwrap_or(config.oidc.redirect_host),
        client_id: env
          .cicada_oidc_client_id
          .unwrap_or(config.oidc.client_id),
        client_secret: env
          .cicada_oidc_client_secret
          .unwrap_or(config.oidc.client_secret),
        use_full_email: env
          .cicada_oidc_use_full_email
          .unwrap_or(config.oidc.use_full_email),
        additional_audiences: env
          .cicada_oidc_additional_audiences
          .unwrap_or(config.oidc.additional_audiences),
      },
      github_oauth: NamedOauthConfig {
        enabled: env
          .cicada_github_oauth_enabled
          .unwrap_or(config.github_oauth.enabled),
        client_id: env
          .cicada_github_oauth_client_id
          .unwrap_or(config.github_oauth.client_id),
        client_secret: env
          .cicada_github_oauth_client_secret
          .unwrap_or(config.github_oauth.client_secret),
      },
      google_oauth: NamedOauthConfig {
        enabled: env
          .cicada_google_oauth_enabled
          .unwrap_or(config.google_oauth.enabled),
        client_id: env
          .cicada_google_oauth_client_id
          .unwrap_or(config.google_oauth.client_id),
        client_secret: env
          .cicada_google_oauth_client_secret
          .unwrap_or(config.google_oauth.client_secret),
      },
      auth_rate_limit_disabled: env
        .cicada_auth_rate_limit_disabled
        .unwrap_or(config.auth_rate_limit_disabled),
      auth_rate_limit_max_attempts: env
        .cicada_auth_rate_limit_max_attempts
        .unwrap_or(config.auth_rate_limit_max_attempts),
      auth_rate_limit_window_seconds: env
        .cicada_auth_rate_limit_window_seconds
        .unwrap_or(config.auth_rate_limit_window_seconds),
      cors_allowed_origins: env
        .cicada_cors_allowed_origins
        .unwrap_or(config.cors_allowed_origins),
      logging: LogConfig {
        level: env
          .cicada_logging_level
          .unwrap_or(config.logging.level),
        stdio: env
          .cicada_logging_stdio
          .unwrap_or(config.logging.stdio),
        pretty: env
          .cicada_logging_pretty
          .unwrap_or(config.logging.pretty),
        location: env
          .cicada_logging_location
          .unwrap_or(config.logging.location),
        ansi: env.cicada_logging_ansi.unwrap_or(config.logging.ansi),
        otlp_endpoint: env
          .cicada_logging_otlp_endpoint
          .unwrap_or(config.logging.otlp_endpoint),
        opentelemetry_service_name: env
          .cicada_logging_opentelemetry_service_name
          .unwrap_or(config.logging.opentelemetry_service_name),
        opentelemetry_scope_name: env
          .cicada_logging_opentelemetry_scope_name
          .unwrap_or(config.logging.opentelemetry_scope_name),
      },
      pretty_startup_config: env
        .cicada_pretty_startup_config
        .unwrap_or(config.pretty_startup_config),
      unsafe_unsanitized_startup_config: env
        .cicada_unsafe_unsanitized_startup_config
        .unwrap_or(config.unsafe_unsanitized_startup_config),
      ssl_enabled: env
        .cicada_ssl_enabled
        .unwrap_or(config.ssl_enabled),
      ssl_key_file: env
        .cicada_ssl_key_file
        .unwrap_or(config.ssl_key_file),
      ssl_cert_file: env
        .cicada_ssl_cert_file
        .unwrap_or(config.ssl_cert_file),
      ui_path: env.cicada_ui_path.unwrap_or(config.ui_path),
    }
  })
}
