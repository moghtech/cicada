//! # Configuring the Cicada Periphery Agent
//!
//! The periphery configuration is passed in three ways:
//! 1. Environment Variables ([Env])
//! 2. Configuration File ([PeripheryConfig])
//!
//! The final configuration is built by combining parameters
//! passed through the different methods. The priority of the args is
//! strictly hierarchical, meaning params passed with [Env] have top priority,
//! followed by those passed in the configuration file.
//!

use std::path::PathBuf;

use serde::Deserialize;

use crate::entities::config::logger::{
  LogConfig, LogLevel, StdioLogMode,
};

/// # Periphery Environment Variables
///
/// The variables should be passed in the traditional `UPPER_SNAKE_CASE` format,
/// although the lower case format can still be parsed.
#[derive(Debug, Deserialize)]
pub struct Env {
  /// Specify the config paths (files or folders) used to build up the
  /// final [PeripheryConfig].
  /// If not provided, will use Default config.
  ///
  /// Note. This is overridden if the equivalent arg is passed in [CliArgs].
  #[serde(default, alias = "periphery_config_path")]
  pub periphery_config_paths: Vec<PathBuf>,
  /// If specifying folders, use this to narrow down which
  /// files will be matched to parse into the final [PeripheryConfig].
  /// Only files inside the folders which have names containing a keywords
  /// provided to `config_keywords` will be included.
  /// Keywords support wildcard matching syntax.
  ///
  /// Note. This is overridden if the equivalent arg is passed in [CliArgs].
  #[serde(
    default = "super::default_config_keywords",
    alias = "periphery_config_keyword"
  )]
  pub periphery_config_keywords: Vec<String>,

  /// Will merge nested config object (eg. secrets, providers) across multiple
  /// config files. Default: `true`
  ///
  /// Note. This is overridden if the equivalent arg is passed in [CliArgs].
  #[serde(default = "super::default_merge_nested_config")]
  pub periphery_merge_nested_config: bool,

  /// Will extend config arrays (eg. `allowed_ips`, `passkeys`) across multiple config files.
  /// Default: `true`
  ///
  /// Note. This is overridden if the equivalent arg is passed in [CliArgs].
  #[serde(default = "super::default_extend_config_arrays")]
  pub periphery_extend_config_arrays: bool,

  /// Override `core_address`
  pub periphery_core_address: Option<String>,
  /// Override `core_tls_insecure_skip_verify`
  pub periphery_core_tls_insecure_skip_verify: Option<bool>,
  /// Override `filesystem_root`
  pub periphery_filesystem_root: Option<PathBuf>,
  /// Override `filesystems`
  #[serde(alias = "periphery_filesystem")]
  pub periphery_filesystems: Option<Vec<String>>,

  // LOGGING
  /// Override `logging.level`
  pub periphery_logging_level: Option<LogLevel>,
  /// Override `logging.stdio`
  pub periphery_logging_stdio: Option<StdioLogMode>,
  /// Override `logging.pretty`
  pub periphery_logging_pretty: Option<bool>,
  /// Override `logging.location`
  pub periphery_logging_location: Option<bool>,
  /// Override `logging.ansi`
  pub periphery_logging_ansi: Option<bool>,
  /// Override `logging.otlp_endpoint`
  pub periphery_logging_otlp_endpoint: Option<String>,
  /// Override `logging.opentelemetry_service_name`
  pub periphery_logging_opentelemetry_service_name: Option<String>,
  /// Override `logging.opentelemetry_scope_name`
  pub periphery_logging_opentelemetry_scope_name: Option<String>,
  /// Override `pretty_startup_config`
  pub periphery_pretty_startup_config: Option<bool>,
}

/// # Periphery Configuration File
///
/// Refer to the [example file](https://github.com/moghtech/komodo/blob/main/config/periphery.config.toml) for a full example.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct PeripheryConfig {
  /// Address of Cicada Core
  #[serde(default)]
  pub core_address: String,

  /// Allow Periphery to connect to Core
  /// without validating the Core certs
  #[serde(default)]
  pub core_tls_insecure_skip_verify: bool,

  /// Specify the default filesystem root when
  /// mount points are ommitted or relative.
  ///
  /// Generally this only needs to be changed for development.
  ///
  /// Default: `/cicada`
  #[serde(default = "default_filesystem_root")]
  pub filesystem_root: PathBuf,

  /// Specify the filesystems to mount using `name:/path/to/mount` syntax.
  ///
  /// Relative paths are relative to the filesystem root.
  /// If the path is ommitted, will mount to $filesystem_root/$name.
  ///
  /// Example:
  ///
  /// ```toml
  /// filesystems = [
  ///   "app1",
  ///   "app2:/custom/app2",
  /// ]
  /// ```
  #[serde(default)]
  pub filesystems: Vec<String>,

  /// Logging configuration
  #[serde(default)]
  pub logging: LogConfig,
}

fn default_filesystem_root() -> PathBuf {
  PathBuf::from("/cicada")
}
