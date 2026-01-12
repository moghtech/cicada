use std::{path::PathBuf, sync::OnceLock};

use cicada_client::entities::config::{
  logger::{LogConfig, LogLevel},
  periphery::{Env, PeripheryConfig},
};
use colored::Colorize;
use mogh_config::ConfigLoader;
use mogh_pki::key::{RotatableKeyPair, SpkiPublicKey};
use mogh_secret_file::maybe_read_item_from_file;

/// Should call in startup to ensure Periphery errors without valid private key.
pub fn periphery_keys() -> &'static RotatableKeyPair {
  static PERIPHERY_KEYS: OnceLock<RotatableKeyPair> = OnceLock::new();
  PERIPHERY_KEYS.get_or_init(|| {
    RotatableKeyPair::from_private_key_spec(
      mogh_pki::PkiType::OneWay,
      &periphery_config().private_key,
    )
    .unwrap()
  })
}

pub fn core_public_key() -> &'static SpkiPublicKey {
  static CORE_PUBLIC_KEY: OnceLock<SpkiPublicKey> = OnceLock::new();
  CORE_PUBLIC_KEY.get_or_init(|| {
    SpkiPublicKey::from_spec(&periphery_config().core_public_key)
      .unwrap()
  })
}

pub fn periphery_config() -> &'static PeripheryConfig {
  static PERIPHERY_CONFIG: OnceLock<PeripheryConfig> =
    OnceLock::new();
  PERIPHERY_CONFIG.get_or_init(|| {
    let env: Env = envy::from_env()
      .expect("failed to parse periphery environment");

    println!("{env:#?}");

    let config_paths = env.periphery_config_paths;

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
          .periphery_config_keywords
          .iter()
          .map(String::as_str)
          .collect::<Vec<_>>(),
        include_file_name: ".peripheryinclude",
        merge_nested: env.periphery_merge_nested_config,
        extend_array: env.periphery_extend_config_arrays,
        debug_print: env
          .periphery_logging_level
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
        .periphery_core_address
        .unwrap_or(config.core_address),
      core_tls_insecure_skip_verify: env
        .periphery_core_tls_insecure_skip_verify
        .unwrap_or(config.core_tls_insecure_skip_verify),
      private_key: maybe_read_item_from_file(
        env.periphery_private_key_file,
        env.periphery_private_key,
      )
      .unwrap_or(config.private_key),
      core_public_key: env
        .periphery_core_public_key
        .unwrap_or(config.core_public_key),
      filesystem_root: env
        .periphery_filesystem_root
        .unwrap_or(config.filesystem_root),
      filesystems: env
        .periphery_filesystems
        .unwrap_or(config.filesystems),
      logging: LogConfig {
        level: env
          .periphery_logging_level
          .unwrap_or(config.logging.level),
        stdio: env
          .periphery_logging_stdio
          .unwrap_or(config.logging.stdio),
        pretty: env
          .periphery_logging_pretty
          .unwrap_or(config.logging.pretty),
        location: env
          .periphery_logging_location
          .unwrap_or(config.logging.location),
        ansi: env
          .periphery_logging_ansi
          .unwrap_or(config.logging.ansi),
        otlp_endpoint: env
          .periphery_logging_otlp_endpoint
          .unwrap_or(config.logging.otlp_endpoint),
        opentelemetry_service_name: env
          .periphery_logging_opentelemetry_service_name
          .unwrap_or(config.logging.opentelemetry_service_name),
        opentelemetry_scope_name: env
          .periphery_logging_opentelemetry_scope_name
          .unwrap_or(config.logging.opentelemetry_scope_name),
      },
      pretty_startup_config: env
        .periphery_pretty_startup_config
        .unwrap_or(config.pretty_startup_config),
      unsafe_unsanitized_startup_config: env
        .periphery_unsafe_unsanitized_startup_config
        .unwrap_or(config.unsafe_unsanitized_startup_config),
    }
  })
}
