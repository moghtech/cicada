use std::{path::PathBuf, sync::OnceLock};

use cicada_client::entities::{
  config::periphery::{Env, PeripheryConfig},
  logger::{LogConfig, LogLevel},
};
use colored::Colorize;
use config::ConfigLoader;

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
    }
  })
}
