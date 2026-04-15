use std::{path::PathBuf, sync::OnceLock};

use mogh_auth_client::config::{
  NamedOauthConfig, OidcConfig, empty_or_redacted,
};
use serde::Deserialize;

use crate::entities::{
  Timelength,
  config::logger::{LogConfig, LogLevel, StdioLogMode},
};

/// # Cicada Core Environment Variables
///
/// You can override any fields of the [CoreConfig] by passing the associated
/// environment variable. The variables should be passed in the traditional `UPPER_SNAKE_CASE` format,
/// although the lower case format can still be parsed.
///
/// *Note.* The Cicada Core docker image includes the default core configuration found at
/// [https://github.com/moghtech/cicada/blob/main/config/core.config.toml](https://github.com/moghtech/cicada/blob/main/config/core.config.toml).
/// To configure the core api, you can either mount your own custom configuration file to
/// `/config/config.toml` inside the container,
/// or simply override whichever fields you need using the environment.
#[derive(Debug, Clone, Deserialize)]
pub struct Env {
  /// Specify a custom config path for the core config toml.
  /// Default: `/config/config.toml`
  #[serde(
    default = "default_core_config_paths",
    alias = "cicada_config_path"
  )]
  pub cicada_config_paths: Vec<PathBuf>,
  /// If specifying folders, use this to narrow down which
  /// files will be matched to parse into the final [PeripheryConfig].
  /// Only files inside the folders which have names containing a keywords
  /// provided to `config_keywords` will be included.
  /// Keywords support wildcard matching syntax.
  #[serde(
    default = "super::default_config_keywords",
    alias = "cicada_config_keyword"
  )]
  pub cicada_config_keywords: Vec<String>,
  /// Will merge nested config objects across multiple
  /// config files. Default: `true`
  #[serde(default = "super::default_merge_nested_config")]
  pub cicada_merge_nested_config: bool,
  /// Will extend config arrays across multiple config files.
  /// Default: `true`
  #[serde(default = "super::default_extend_config_arrays")]
  pub cicada_extend_config_arrays: bool,
  /// Print some extra logs on startup to debug config loading issues.
  #[serde(default)]
  pub cicada_config_debug: bool,

  /// Override `title`
  pub cicada_title: Option<String>,
  /// Override `host`
  pub cicada_host: Option<String>,
  /// Override `port`
  pub cicada_port: Option<u16>,
  /// Override `bind_ip`
  pub cicada_bind_ip: Option<String>,

  /// Override `jwt_secret`
  pub cicada_jwt_secret: Option<String>,
  /// Override `jwt_secret` from file
  pub cicada_jwt_secret_file: Option<PathBuf>,
  /// Override `jwt_ttl`
  pub cicada_jwt_ttl: Option<Timelength>,

  /// Override `private_key`
  pub cicada_private_key: Option<String>,
  /// Override `private_key` with file
  pub cicada_private_key_file: Option<PathBuf>,

  /// Override `database.uri`
  pub cicada_database_uri: Option<String>,
  /// Override `database.username`
  pub cicada_database_username: Option<String>,
  /// Override `database.username` with file
  pub cicada_database_username_file: Option<PathBuf>,
  /// Override `database.password`
  pub cicada_database_password: Option<String>,
  /// Override `database.password` with file
  pub cicada_database_password_file: Option<PathBuf>,
  /// Override `database.namespace`
  pub cicada_database_namespace: Option<String>,
  /// Override `database.db_name`
  pub cicada_database_db_name: Option<String>,

  /// Override `local_auth`
  pub cicada_local_auth: Option<bool>,
  /// Override `disable_user_registration`
  pub cicada_disable_user_registration: Option<bool>,
  /// Override `enable_new_users`
  pub cicada_enable_new_users: Option<bool>,
  /// Override `lock_login_credentials_for`
  pub cicada_lock_login_credentials_for: Option<Vec<String>>,

  /// Override `oidc_enabled`
  pub cicada_oidc_enabled: Option<bool>,
  /// Override `oidc_provider`
  pub cicada_oidc_provider: Option<String>,
  /// Override `oidc_redirect_host`
  pub cicada_oidc_redirect_host: Option<String>,
  /// Override `oidc_client_id`
  pub cicada_oidc_client_id: Option<String>,
  /// Override `oidc_client_id` from file
  pub cicada_oidc_client_id_file: Option<PathBuf>,
  /// Override `oidc_client_secret`
  pub cicada_oidc_client_secret: Option<String>,
  /// Override `oidc_client_secret` from file
  pub cicada_oidc_client_secret_file: Option<PathBuf>,
  /// Override `oidc_use_full_email`
  pub cicada_oidc_use_full_email: Option<bool>,
  /// Override `oidc_additional_audiences`
  pub cicada_oidc_additional_audiences: Option<Vec<String>>,
  /// Override `oidc_additional_audiences` from file
  pub cicada_oidc_additional_audiences_file: Option<PathBuf>,
  /// Override `oidc_auto_redirect` from file
  pub cicada_oidc_auto_redirect: Option<bool>,

  /// Override `github_oauth.enabled`
  pub cicada_github_oauth_enabled: Option<bool>,
  /// Override `github_oauth.client_id`
  #[serde(alias = "cicada_github_oauth_id")]
  pub cicada_github_oauth_client_id: Option<String>,
  /// Override `github_oauth.client_id` from file
  #[serde(alias = "cicada_github_oauth_id_file")]
  pub cicada_github_oauth_client_id_file: Option<PathBuf>,
  /// Override `github_oauth.secret`
  #[serde(alias = "cicada_github_oauth_secret")]
  pub cicada_github_oauth_client_secret: Option<String>,
  /// Override `github_oauth.secret` from file
  #[serde(alias = "cicada_github_oauth_secret_file")]
  pub cicada_github_oauth_client_secret_file: Option<PathBuf>,

  /// Override `google_oauth.enabled`
  pub cicada_google_oauth_enabled: Option<bool>,
  /// Override `google_oauth.client_id`
  #[serde(alias = "cicada_google_oauth_id")]
  pub cicada_google_oauth_client_id: Option<String>,
  /// Override `google_oauth.client_id` from file
  #[serde(alias = "cicada_google_oauth_id_file")]
  pub cicada_google_oauth_client_id_file: Option<PathBuf>,
  /// Override `google_oauth.secret`
  #[serde(alias = "cicada_google_oauth_secret")]
  pub cicada_google_oauth_client_secret: Option<String>,
  /// Override `google_oauth.secret` from file
  #[serde(alias = "cicada_google_oauth_secret_file")]
  pub cicada_google_oauth_client_secret_file: Option<PathBuf>,

  /// Override `auth_rate_limit_disabled`
  pub cicada_auth_rate_limit_disabled: Option<bool>,
  /// Override `auth_rate_limit_max_attempts`
  pub cicada_auth_rate_limit_max_attempts: Option<u16>,
  /// Override `auth_rate_limit_window_seconds`
  pub cicada_auth_rate_limit_window_seconds: Option<u64>,

  /// Override `cors_allowed_origins`
  pub cicada_cors_allowed_origins: Option<Vec<String>>,
  /// Override `cors_allow_credentials`
  pub cicada_cors_allow_credentials: Option<bool>,
  /// Override `session_allow_cross_site`
  pub cicada_session_allow_cross_site: Option<bool>,

  /// Override `logging.level`
  pub cicada_logging_level: Option<LogLevel>,
  /// Override `logging.stdio`
  pub cicada_logging_stdio: Option<StdioLogMode>,
  /// Override `logging.pretty`
  pub cicada_logging_pretty: Option<bool>,
  /// Override `logging.location`
  pub cicada_logging_location: Option<bool>,
  /// Override `logging.ansi`
  pub cicada_logging_ansi: Option<bool>,
  /// Override `logging.timestamps`
  pub cicada_logging_timestamps: Option<bool>,
  /// Override `logging.otlp_endpoint`
  pub cicada_logging_otlp_endpoint: Option<String>,
  /// Override `logging.opentelemetry_service_name`
  pub cicada_logging_opentelemetry_service_name: Option<String>,
  /// Override `logging.opentelemetry_scope_name`
  pub cicada_logging_opentelemetry_scope_name: Option<String>,
  /// Override `pretty_startup_config`
  pub cicada_pretty_startup_config: Option<bool>,
  /// Override `unsafe_unsanitized_startup_config`
  pub cicada_unsafe_unsanitized_startup_config: Option<bool>,

  /// Override `ssl_enabled`.
  pub cicada_ssl_enabled: Option<bool>,
  /// Override `ssl_key_file`
  pub cicada_ssl_key_file: Option<String>,
  /// Override `ssl_cert_file`
  pub cicada_ssl_cert_file: Option<String>,

  /// Override `ui_path`
  pub cicada_ui_path: Option<String>,
  /// Override `ui_index_force_no_cache`
  pub cicada_ui_index_force_no_cache: Option<bool>,
}

fn default_core_config_paths() -> Vec<PathBuf> {
  vec![PathBuf::from("/config")]
}

/// # Core Configuration File
///
/// The Core API initializes it's configuration by reading the environment,
/// parsing the [CoreConfig] schema from the file path specified by `env.cicada_config_path`,
/// and then applying any config field overrides specified in the environment.
///
/// *Note.* The Cicada Core docker image includes the default core configuration found at
/// [https://github.com/moghtech/cicada/blob/main/config/core.config.toml](https://github.com/moghtech/cicada/blob/main/config/core.config.toml).
/// To configure the core api, you can either mount your own custom configuration file to
/// `/config/config.toml` inside the container,
/// or simply override whichever fields you need using the environment.
///
/// Refer to the [example file](https://github.com/moghtech/cicada/blob/main/config/core.config.toml) for a full example.
#[derive(Debug, Clone, Deserialize)]
pub struct CoreConfig {
  // ===========
  // = General =
  // ===========
  /// The title of this Cicada Core deployment. Will be used in the browser page title.
  /// Default: 'Cicada'
  #[serde(default = "default_title")]
  pub title: String,

  /// The host to use with oauth redirect url, whatever host
  /// the user hits to access Cicada. eg `https://cicada.domain.com`.
  /// Only used if oauth used without user specifying redirect url themselves.
  #[serde(default = "default_host")]
  pub host: String,

  /// Port the core web server runs on.
  /// Default: 9220.
  #[serde(default = "default_core_port")]
  pub port: u16,

  /// IP address the core server binds to.
  /// Default: [::].
  #[serde(default = "default_core_bind_ip")]
  pub bind_ip: String,

  /// Private key to use with Noise handshake to authenticate with Periphery agents.
  ///
  /// Supports openssl generated pem file, `openssl genpkey -algorithm X25519 -out private.key`.
  /// To load from file, use `private_key = "file:/path/to/private.key"`.
  ///
  /// If a file is specified and does not exist, will try to generate one at the path
  /// and use it going forward.
  ///
  /// Default: file:/config/keys/core.key
  #[serde(default = "default_private_key")]
  pub private_key: String,

  /// Configure database connection
  #[serde(default)]
  pub database: DatabaseConfig,

  // ================
  // = Auth / Login =
  // ================
  /// Enable login with local auth
  #[serde(default)]
  pub local_auth: bool,

  /// New users will be automatically enabled.
  #[serde(default)]
  pub enable_new_users: bool,

  /// Normally new users will be registered, but not enabled until an Admin enables them.
  /// With `disable_user_registration = true`, only the first user to sign up will be registered as a user.
  #[serde(default)]
  pub disable_user_registration: bool,

  /// List of usernames for which the update username / password
  /// APIs are disabled. Used by demo to lock the 'demo' : 'demo' login.
  ///
  /// To lock the api for all users, use `lock_login_credentials_for = ["__ALL__"]`
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub lock_login_credentials_for: Vec<String>,

  /// Optionally provide a specific jwt secret.
  /// Passing nothing or an empty string will cause one to be generated.
  /// Default: "" (empty string)
  #[serde(default)]
  pub jwt_secret: String,

  /// Control how long distributed JWT remain valid for.
  /// Default: `1-day`.
  #[serde(default = "default_jwt_ttl")]
  pub jwt_ttl: Timelength,

  /// OIDC login configuration
  #[serde(default)]
  pub oidc: OidcConfig,

  /// Github oauth login configuration
  #[serde(default)]
  pub github_oauth: NamedOauthConfig,

  /// Google oauth login configuration
  #[serde(default)]
  pub google_oauth: NamedOauthConfig,

  // =================
  // = Rate Limiting =
  // =================
  /// Disable the auth rate limiter.
  #[serde(default)]
  pub auth_rate_limit_disabled: bool,

  /// Set the max allowed attempts per IP
  #[serde(default = "default_auth_rate_limit_max_attempts")]
  pub auth_rate_limit_max_attempts: u16,

  #[serde(default = "default_auth_rate_limit_window_seconds")]
  pub auth_rate_limit_window_seconds: u64,

  // =======
  // = CORS =
  // =======
  /// List of additional CORS allowed origins.
  /// Example: `["https://cicada.example.com", "https://app.example.com"]`.
  #[serde(default)]
  pub cors_allowed_origins: Vec<String>,

  /// Allow credentials from additional origins.
  #[serde(default)]
  pub cors_allow_credentials: bool,

  /// Use SameSite=None (actually allows samesite) instead of SameSite=Lax.
  /// The third option, SameSite=Strict, won't work with external login,
  /// as the session cookie will be lost on redirect with auth provider.
  #[serde(default)]
  pub session_allow_cross_site: bool,

  // ===========
  // = Logging =
  // ===========
  /// Logging configuration
  #[serde(default)]
  pub logging: LogConfig,

  /// Pretty-log (multi-line) the startup config
  /// for easier human readability.
  #[serde(default)]
  pub pretty_startup_config: bool,

  /// Unsafe: logs unsanitized config on startup,
  /// in order to verify everything is being
  /// passed correctly.
  #[serde(default)]
  pub unsafe_unsanitized_startup_config: bool,

  // =======
  // = SSL =
  // =======
  /// Whether to enable ssl.
  #[serde(default)]
  pub ssl_enabled: bool,

  /// Path to the ssl key.
  /// Default: `/config/ssl/key.pem`.
  #[serde(default = "default_ssl_key_file")]
  pub ssl_key_file: String,

  /// Path to the ssl cert.
  /// Default: `/config/ssl/cert.pem`.
  #[serde(default = "default_ssl_cert_file")]
  pub ssl_cert_file: String,

  // =======
  // = DEV =
  // =======
  /// The path to the built ui static folder.
  #[serde(default = "default_ui_path")]
  pub ui_path: String,

  /// Force the `index.html` to served with
  /// 'Cache-Content: no-cache' header instead
  /// of using content hash as ETag.
  #[serde(default)]
  pub ui_index_force_no_cache: bool,
}

fn default_title() -> String {
  String::from("Cicada")
}

fn default_host() -> String {
  String::from("https://cicada.example.com")
}

fn default_core_port() -> u16 {
  9220
}

fn default_core_bind_ip() -> String {
  "[::]".to_string()
}

fn default_private_key() -> String {
  String::from("file:/config/keys/core.key")
}

fn default_jwt_ttl() -> Timelength {
  Timelength::OneDay
}

fn default_auth_rate_limit_max_attempts() -> u16 {
  5
}

fn default_auth_rate_limit_window_seconds() -> u64 {
  15
}

fn default_ssl_key_file() -> String {
  "/config/ssl/key.pem".to_string()
}

fn default_ssl_cert_file() -> String {
  "/config/ssl/cert.pem".to_string()
}

fn default_ui_path() -> String {
  "/app/ui".to_string()
}

impl Default for CoreConfig {
  fn default() -> Self {
    Self {
      title: default_title(),
      host: default_host(),
      port: default_core_port(),
      bind_ip: default_core_bind_ip(),
      jwt_secret: Default::default(),
      jwt_ttl: default_jwt_ttl(),
      private_key: default_private_key(),
      database: Default::default(),
      local_auth: Default::default(),
      disable_user_registration: Default::default(),
      enable_new_users: Default::default(),
      lock_login_credentials_for: Default::default(),
      oidc: Default::default(),
      github_oauth: Default::default(),
      google_oauth: Default::default(),
      auth_rate_limit_disabled: Default::default(),
      auth_rate_limit_max_attempts:
        default_auth_rate_limit_max_attempts(),
      auth_rate_limit_window_seconds:
        default_auth_rate_limit_window_seconds(),
      cors_allowed_origins: Default::default(),
      cors_allow_credentials: Default::default(),
      session_allow_cross_site: Default::default(),
      logging: Default::default(),
      pretty_startup_config: Default::default(),
      unsafe_unsanitized_startup_config: Default::default(),
      ssl_enabled: Default::default(),
      ssl_key_file: default_ssl_key_file(),
      ssl_cert_file: default_ssl_cert_file(),
      ui_path: default_ui_path(),
      ui_index_force_no_cache: Default::default(),
    }
  }
}

impl CoreConfig {
  pub fn sanitized(&self) -> CoreConfig {
    let mut config = self.clone();
    config.oidc.sanitize();
    config.github_oauth.sanitize();
    config.google_oauth.sanitize();
    CoreConfig {
      title: config.title,
      host: config.host,
      port: config.port,
      bind_ip: config.bind_ip,
      jwt_secret: empty_or_redacted(&config.jwt_secret),
      jwt_ttl: config.jwt_ttl,
      private_key: if self.private_key.starts_with("file:") {
        self.private_key.clone()
      } else {
        empty_or_redacted(&self.private_key)
      },
      database: config.database.sanitized(),
      local_auth: config.local_auth,
      disable_user_registration: config.disable_user_registration,
      enable_new_users: config.enable_new_users,
      lock_login_credentials_for: config.lock_login_credentials_for,
      oidc: config.oidc,
      github_oauth: config.github_oauth,
      google_oauth: config.google_oauth,
      logging: config.logging,
      pretty_startup_config: config.pretty_startup_config,
      unsafe_unsanitized_startup_config: config
        .unsafe_unsanitized_startup_config,
      auth_rate_limit_disabled: config.auth_rate_limit_disabled,
      auth_rate_limit_max_attempts: config
        .auth_rate_limit_max_attempts,
      auth_rate_limit_window_seconds: config
        .auth_rate_limit_window_seconds,
      cors_allowed_origins: config.cors_allowed_origins,
      cors_allow_credentials: config.cors_allow_credentials,
      session_allow_cross_site: config.session_allow_cross_site,
      ssl_enabled: config.ssl_enabled,
      ssl_key_file: config.ssl_key_file,
      ssl_cert_file: config.ssl_cert_file,
      ui_path: config.ui_path,
      ui_index_force_no_cache: config.ui_index_force_no_cache,
    }
  }

  // pub fn oidc_enabled(&self) -> bool {
  //   self.oidc_enabled
  //     && !self.oidc_provider.is_empty()
  //     && !self.oidc_client_id.is_empty()
  // }
}

/// Provide database connection information.
///
/// Cicada uses [SurrealDB](https://surrealdb.com/)
/// for persistance.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct DatabaseConfig {
  /// Surreal uri string:
  /// - Local: `rocksdb:/database`
  /// - Remote: `wss://my.surreal.db`
  #[serde(
    default = "default_database_uri",
    skip_serializing_if = "String::is_empty"
  )]
  pub uri: String,
  /// Database user username
  #[serde(default, skip_serializing_if = "String::is_empty")]
  pub username: String,
  /// Database user password
  #[serde(default, skip_serializing_if = "String::is_empty")]
  pub password: String,
  /// Namespace to use. default: `cicada`
  #[serde(default = "default_database_namespace")]
  pub namespace: String,
  /// Database name. Which database to create the tables in.
  /// Default: `cicada`.
  #[serde(default = "default_database_db_name")]
  pub db_name: String,
}

fn default_database_uri() -> String {
  String::from("rocksdb:/database")
}

fn default_database_namespace() -> String {
  "cicada".to_string()
}

fn default_database_db_name() -> String {
  "cicada".to_string()
}

impl Default for DatabaseConfig {
  fn default() -> Self {
    Self {
      uri: default_database_uri(),
      username: Default::default(),
      password: Default::default(),
      namespace: default_database_namespace(),
      db_name: default_database_db_name(),
    }
  }
}

fn default_database_config() -> &'static DatabaseConfig {
  static DEFAULT_DATABASE_CONFIG: OnceLock<DatabaseConfig> =
    OnceLock::new();
  DEFAULT_DATABASE_CONFIG.get_or_init(Default::default)
}

impl DatabaseConfig {
  pub fn sanitized(&self) -> DatabaseConfig {
    DatabaseConfig {
      uri: self.uri.clone(),
      username: empty_or_redacted(&self.username),
      password: empty_or_redacted(&self.password),
      namespace: self.namespace.clone(),
      db_name: self.db_name.clone(),
    }
  }

  pub fn is_default(&self) -> bool {
    self == default_database_config()
  }
}

#[cfg(feature = "core")]
impl mogh_server::ServerConfig for &CoreConfig {
  fn bind_ip(&self) -> &str {
    &self.bind_ip
  }
  fn port(&self) -> u16 {
    self.port
  }
  fn ssl_enabled(&self) -> bool {
    self.ssl_enabled
  }
  fn ssl_key_file(&self) -> &str {
    &self.ssl_key_file
  }
  fn ssl_cert_file(&self) -> &str {
    &self.ssl_cert_file
  }
}

#[cfg(feature = "core")]
impl mogh_server::cors::CorsConfig for &CoreConfig {
  fn allowed_origins(&self) -> &[String] {
    &self.cors_allowed_origins
  }
  fn allow_credentials(&self) -> bool {
    self.cors_allow_credentials
  }
}

#[cfg(feature = "core")]
impl mogh_server::session::SessionConfig for &CoreConfig {
  fn host(&self) -> &str {
    &self.host
  }
  fn host_env_field(&self) -> &str {
    "CICADA_HOST"
  }
  fn expiry_seconds(&self) -> i64 {
    60
  }
  fn allow_cross_site(&self) -> bool {
    self.session_allow_cross_site
  }
}
