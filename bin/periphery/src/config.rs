use std::{path::PathBuf, sync::OnceLock};

use cicada_client::entities::config::{
  logger::{LogConfig, LogLevel},
  periphery::{Env, PeripheryConfig},
};
use colored::Colorize;
use mogh_config::ConfigLoader;
use mogh_pki::{RotatableKeyPair, SpkiPublicKey};
use mogh_secret_file::maybe_read_item_from_file;

/// Should call in startup to ensure Periphery errors without valid private key.
pub fn periphery_keys() -> &'static RotatableKeyPair {
  static PERIPHERY_KEYS: OnceLock<RotatableKeyPair> = OnceLock::new();
  PERIPHERY_KEYS.get_or_init(|| {
    RotatableKeyPair::from_private_key_spec(
      mogh_pki::PkiKind::OneWay,
      &periphery_config().private_key,
    )
    .unwrap()
  })
}

pub fn core_public_key() -> Option<&'static SpkiPublicKey> {
  static CORE_PUBLIC_KEY: OnceLock<Option<SpkiPublicKey>> =
    OnceLock::new();
  CORE_PUBLIC_KEY
    .get_or_init(|| {
      if periphery_config().core_public_key.is_empty() {
        return None;
      }
      Some(
        SpkiPublicKey::from_spec(&periphery_config().core_public_key)
          .unwrap(),
      )
    })
    .as_ref()
}

pub fn periphery_config() -> &'static PeripheryConfig {
  static PERIPHERY_CONFIG: OnceLock<PeripheryConfig> =
    OnceLock::new();
  PERIPHERY_CONFIG.get_or_init(|| {
    let env: Env = envy::from_env()
      .expect("failed to parse cicada periphery environment");

    let config_paths = env.cicada_config_paths;

    println!("{config_paths:?}");

    let config = if config_paths.is_empty() {
      println!(
        "{}: No config paths found, using default config",
        "INFO".green(),
      );
      PeripheryConfig::default()
    } else {
      (ConfigLoader {
        paths: &config_paths
          .iter()
          .map(PathBuf::as_path)
          .collect::<Vec<_>>(),
        match_wildcards: &env
          .cicada_config_keywords
          .iter()
          .map(String::as_str)
          .collect::<Vec<_>>(),
        include_file_name: ".cicadainclude",
        merge_nested: env.cicada_merge_nested_config,
        extend_array: env.cicada_extend_config_arrays,
        debug_print: env
          .cicada_logging_level
          .map(|level| {
            matches!(level, LogLevel::Debug | LogLevel::Trace)
          })
          .unwrap_or_default(),
      })
      .load()
      .expect("failed at parsing config from paths")
    };

    PeripheryConfig {
      core_address: env
        .cicada_core_address
        .unwrap_or(config.core_address),
      core_tls_insecure_skip_verify: env
        .cicada_core_tls_insecure_skip_verify
        .unwrap_or(config.core_tls_insecure_skip_verify),
      private_key: maybe_read_item_from_file(
        env.cicada_private_key_file,
        env.cicada_private_key,
      )
      .unwrap_or(config.private_key),
      onboarding_key: maybe_read_item_from_file(
        env.cicada_onboarding_key_file,
        env.cicada_onboarding_key,
      )
      .or(config.onboarding_key),
      device_name: env
        .cicada_device_name
        .unwrap_or(config.device_name),
      core_public_key: env
        .cicada_core_public_key
        .unwrap_or(config.core_public_key),
      default_mount_root: env
        .cicada_default_mount_root
        .unwrap_or(config.default_mount_root),
      filesystems: env
        .cicada_filesystems
        .unwrap_or(config.filesystems),
      allow_uids: env.cicada_allow_uids.unwrap_or(config.allow_uids),
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
    }
  })
}
