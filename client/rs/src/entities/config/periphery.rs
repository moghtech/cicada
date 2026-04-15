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

use mogh_auth_client::config::empty_or_redacted;
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
  #[serde(default, alias = "cicada_config_path")]
  pub cicada_config_paths: Vec<PathBuf>,

  /// If specifying folders, use this to narrow down which
  /// files will be matched to parse into the final [PeripheryConfig].
  /// Only files inside the folders which have names containing a keywords
  /// provided to `config_keywords` will be included.
  /// Keywords support wildcard matching syntax.
  ///
  /// Note. This is overridden if the equivalent arg is passed in [CliArgs].
  #[serde(
    default = "super::default_config_keywords",
    alias = "cicada_config_keyword"
  )]
  pub cicada_config_keywords: Vec<String>,

  /// Will merge nested config object (eg. secrets, providers) across multiple
  /// config files. Default: `true`
  ///
  /// Note. This is overridden if the equivalent arg is passed in [CliArgs].
  #[serde(default = "super::default_merge_nested_config")]
  pub cicada_merge_nested_config: bool,

  /// Will extend config arrays (eg. `allowed_ips`, `passkeys`) across multiple config files.
  /// Default: `true`
  ///
  /// Note. This is overridden if the equivalent arg is passed in [CliArgs].
  #[serde(default = "super::default_extend_config_arrays")]
  pub cicada_extend_config_arrays: bool,

  /// Override `core_address`
  pub cicada_core_address: Option<String>,
  /// Override `core_tls_insecure_skip_verify`
  pub cicada_core_tls_insecure_skip_verify: Option<bool>,

  /// Override `private_key`
  pub cicada_private_key: Option<String>,
  /// Override `private_key` with file
  pub cicada_private_key_file: Option<PathBuf>,
  /// Override `onboarding_key`
  pub cicada_onboarding_key: Option<String>,
  /// Override `onboarding_key` from file
  pub cicada_onboarding_key_file: Option<PathBuf>,
  /// Override `device_name`
  pub cicada_device_name: Option<String>,
  /// Override `core_public_key`
  pub cicada_core_public_key: Option<String>,

  /// Override `filesystems`
  #[serde(alias = "cicada_filesystem")]
  pub cicada_filesystems: Option<Vec<String>>,
  /// Override `allow_uids`
  #[serde(alias = "cicada_allow_uid")]
  pub cicada_allow_uids: Option<Vec<u32>>,
  /// Override `default_mount_root`
  pub cicada_default_mount_root: Option<PathBuf>,

  // LOGGING
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
}

/// # Periphery Configuration File
///
/// Refer to the [example file](https://github.com/moghtech/cicada/blob/main/config/periphery.config.toml) for a full example.
#[derive(Debug, Clone, Deserialize)]
pub struct PeripheryConfig {
  /// Address of Cicada Core
  #[serde(default)]
  pub core_address: String,

  /// Allow Periphery to connect to Core
  /// without validating the Core certs
  #[serde(default)]
  pub core_tls_insecure_skip_verify: bool,

  /// Private key to use with Noise handshake to authenticate with Cicada Core.
  ///
  /// Supports openssl generated pem file, `openssl genpkey -algorithm X25519 -out private.key`.
  /// To load from file, use `private_key = "file:/path/to/private.key"`.
  ///
  /// If a file is specified and does not exist, will try to generate one at the path
  /// and use it going forward.
  ///
  /// Default: file:/config/keys/periphery.key
  #[serde(default = "default_private_key")]
  pub private_key: String,

  /// Provide an onboarding key to use with the new Device
  /// creation flow.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub onboarding_key: Option<String>,

  /// The device name to onboard as.
  /// Note. This name is only used during onboarding.
  /// Every device needs a unique name paired with public key.
  #[serde(default)]
  pub device_name: String,

  /// Specify the core public key to use with authentication signature.
  /// If not specified, will be retreived from the Core '/public_key' route.
  #[serde(default)]
  pub core_public_key: String,

  /// Specify the filesystems to mount using `filesystem_name|mount=/path/to/mount|uid=$UID|gid=$GID|rw=true` syntax.
  ///
  /// After the `filesystem_name`, the order of the parameters does not matter.
  ///
  /// For the `mount` paramter, relative paths are relative to the default mount root.
  /// If the path is ommitted, will mount to $filesystem_root/$name.
  ///
  /// Example:
  ///
  /// ```toml
  /// default_mount_root = "/cicada"
  /// filesystems = [
  ///   "app1",                                      # mounts app1 to /cicada/app1
  ///   "app2|mount=relative/path",                  # mounts app2 to /cicada/relative/path
  ///   "app3|mount=/custom/app3",                   # mounts app3 to /custom/app3
  ///   "app4|mount=/custom/app4|uid=1000|rw=true",  # mounts app4 to /custom/app4 with files owned by UID 1000 and GID 1000 with rw
  ///   "app5|mount=/custom/app5|uid=1000|gid=999",  # mounts app5 to /custom/app5 with files owned by UID 1000 and GID 999
  /// ]
  /// ```
  #[serde(default)]
  pub filesystems: Vec<String>,

  /// Allow specific UIDs to access the mounted filesystems.
  ///
  /// To allow user other than mount owner rw, must also specify rw=true.
  ///
  /// Example:
  ///
  /// ```toml
  /// allow_uids = [
  ///   "1001",
  ///   "1002|rw=true"
  /// ]
  /// ```
  #[serde(default)]
  pub allow_uids: Vec<u32>,

  /// Specify the default filesystem mount root when
  /// mount points are ommitted or relative.
  ///
  /// Default: `/cicada`
  #[serde(default = "default_mount_root")]
  pub default_mount_root: PathBuf,

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
}

fn default_private_key() -> String {
  String::from("file:/config/keys/periphery.key")
}

fn default_mount_root() -> PathBuf {
  PathBuf::from("/cicada")
}

impl Default for PeripheryConfig {
  fn default() -> Self {
    Self {
      core_address: Default::default(),
      core_tls_insecure_skip_verify: Default::default(),
      private_key: default_private_key(),
      onboarding_key: Default::default(),
      device_name: Default::default(),
      core_public_key: Default::default(),
      default_mount_root: default_mount_root(),
      filesystems: Default::default(),
      allow_uids: Default::default(),
      logging: Default::default(),
      pretty_startup_config: Default::default(),
      unsafe_unsanitized_startup_config: Default::default(),
    }
  }
}

impl PeripheryConfig {
  pub fn sanitized(&self) -> PeripheryConfig {
    PeripheryConfig {
      core_address: self.core_address.clone(),
      core_tls_insecure_skip_verify: self
        .core_tls_insecure_skip_verify,
      private_key: if self.private_key.starts_with("file:") {
        self.private_key.clone()
      } else {
        empty_or_redacted(&self.private_key)
      },
      onboarding_key: self
        .onboarding_key
        .as_ref()
        .map(|key| empty_or_redacted(key)),
      device_name: self.device_name.clone(),
      core_public_key: self.core_public_key.clone(),
      default_mount_root: self.default_mount_root.clone(),
      filesystems: self.filesystems.clone(),
      allow_uids: self.allow_uids.clone(),
      logging: self.logging.clone(),
      pretty_startup_config: self.pretty_startup_config,
      unsafe_unsanitized_startup_config: self
        .unsafe_unsanitized_startup_config,
    }
  }
}
