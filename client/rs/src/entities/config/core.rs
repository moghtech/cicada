use std::{path::PathBuf, sync::OnceLock};

use serde::Deserialize;

use crate::entities::{
  config::empty_or_redacted,
  logger::{LogConfig, LogLevel, StdioLogMode},
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

  /// Override `cors_allowed_origins`
  pub cicada_cors_allowed_origins: Option<Vec<String>>,
  /// Override `cors_allow_credentials`
  pub cicada_cors_allow_credentials: Option<bool>,

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
  pub cicada_ssl_key_file: Option<PathBuf>,
  /// Override `ssl_cert_file`
  pub cicada_ssl_cert_file: Option<PathBuf>,

  /// Override `ui_path`
  pub cicada_ui_path: Option<String>,
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
  /// Default: 9120.
  #[serde(default = "default_core_port")]
  pub port: u16,

  /// IP address the core server binds to.
  /// Default: [::].
  #[serde(default = "default_core_bind_ip")]
  pub bind_ip: String,

  /// Configure database connection
  #[serde(default)]
  pub database: DatabaseConfig,

  // =======
  // = CORS =
  // =======
  /// List of CORS allowed origins.
  /// If empty, allows all origins (`*`).
  /// Production setups should configure this explicitly.
  /// Example: `["https://cicada.example.com", "https://app.example.com"]`.
  #[serde(default)]
  pub cors_allowed_origins: Vec<String>,

  /// Tell CORS to allow credentials in requests.
  /// Used if needed for authentication proxy.
  #[serde(default)]
  pub cors_allow_credentials: bool,

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
  pub ssl_key_file: PathBuf,

  /// Path to the ssl cert.
  /// Default: `/config/ssl/cert.pem`.
  #[serde(default = "default_ssl_cert_file")]
  pub ssl_cert_file: PathBuf,

  // =======
  // = DEV =
  // =======
  /// The path to the built ui static folder.
  #[serde(default = "default_ui_path")]
  pub ui_path: String,
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

fn default_ssl_key_file() -> PathBuf {
  "/config/ssl/key.pem".parse().unwrap()
}

fn default_ssl_cert_file() -> PathBuf {
  "/config/ssl/cert.pem".parse().unwrap()
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
      database: Default::default(),
      cors_allowed_origins: Default::default(),
      cors_allow_credentials: Default::default(),
      logging: Default::default(),
      pretty_startup_config: Default::default(),
      unsafe_unsanitized_startup_config: Default::default(),
      ssl_enabled: Default::default(),
      ssl_key_file: default_ssl_key_file(),
      ssl_cert_file: default_ssl_cert_file(),
      ui_path: default_ui_path(),
    }
  }
}

impl CoreConfig {
  pub fn sanitized(&self) -> CoreConfig {
    let config = self.clone();
    CoreConfig {
      title: config.title,
      host: config.host,
      port: config.port,
      bind_ip: config.bind_ip,
      database: config.database.sanitized(),
      logging: config.logging,
      pretty_startup_config: config.pretty_startup_config,
      unsafe_unsanitized_startup_config: config
        .unsafe_unsanitized_startup_config,
      cors_allowed_origins: config.cors_allowed_origins,
      cors_allow_credentials: config.cors_allow_credentials,
      ssl_enabled: config.ssl_enabled,
      ssl_key_file: config.ssl_key_file,
      ssl_cert_file: config.ssl_cert_file,
      ui_path: config.ui_path,
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
  /// Full mongo uri string, eg. `mongodb://username:password@your.mongo.int:27017`
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
  String::from("rocksdb:/database/database.db")
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
