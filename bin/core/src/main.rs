#[macro_use]
extern crate tracing;

use std::{net::SocketAddr, str::FromStr as _};

use anyhow::Context as _;
use axum_server::tls_rustls::RustlsConfig;
use tracing::Instrument as _;

use crate::config::core_config;

mod api;
mod config;
mod db;

async fn app() -> anyhow::Result<()> {
  dotenvy::dotenv().ok();
  let config = core_config();
  logger::init(&config.logging)?;

  let startup_span = info_span!("CoreStartup");

  async {
    info!("Cicada Core version: v{}", env!("CARGO_PKG_VERSION"));

    rustls::crypto::aws_lc_rs::default_provider()
      .install_default()
      .map_err(|_| {
        anyhow::Error::msg("Failed to install tls crypto provider")
      })?;

    db::init().await?;

    match (
      config.pretty_startup_config,
      config.unsafe_unsanitized_startup_config,
    ) {
      (true, true) => info!("{:#?}", config),
      (true, false) => info!("{:#?}", config.sanitized()),
      (false, true) => info!("{:?}", config),
      (false, false) => info!("{:?}", config.sanitized()),
    }

    anyhow::Ok(())
  }
  .instrument(startup_span)
  .await?;

  let app =
    api::app().into_make_service_with_connect_info::<SocketAddr>();

  let addr = format!("{}:{}", config.bind_ip, config.port);
  let socket_addr = SocketAddr::from_str(&addr)
    .context("Failed to parse listen address")?;

  if config.ssl_enabled {
    info!("🔒 Core SSL Enabled");
    info!("Cicada Core starting on https://{socket_addr}");
    let ssl_config = RustlsConfig::from_pem_file(
      &config.ssl_cert_file,
      &config.ssl_key_file,
    )
    .await
    .context("Invalid ssl cert / key")?;
    axum_server::bind_rustls(socket_addr, ssl_config)
      .serve(app)
      .await
      .context("failed to start https server")
  } else {
    info!("🔓 Core SSL Disabled");
    info!("Cicada Core starting on http://{socket_addr}");
    axum_server::bind(socket_addr)
      .serve(app)
      .await
      .context("failed to start http server")
  }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let mut term_signal = tokio::signal::unix::signal(
    tokio::signal::unix::SignalKind::terminate(),
  )?;
  tokio::select! {
    res = tokio::spawn(app()) => res?,
    _ = term_signal.recv() => Ok(()),
  }
}
