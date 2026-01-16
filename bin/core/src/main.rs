#[macro_use]
extern crate tracing;

use tracing::Instrument as _;

use crate::config::{core_config, core_keys};

mod api;
mod auth;
mod config;
mod db;
mod encryption;

async fn app() -> anyhow::Result<()> {
  dotenvy::dotenv().ok();
  let config = core_config();
  mogh_logger::init(&config.logging)?;

  let startup_span = info_span!("CoreStartup");

  async {
    info!("Cicada Core version: v{}", env!("CARGO_PKG_VERSION"));

    match (
      config.pretty_startup_config,
      config.unsafe_unsanitized_startup_config,
    ) {
      (true, true) => info!("{:#?}", config),
      (true, false) => info!("{:#?}", config.sanitized()),
      (false, true) => info!("{:?}", config),
      (false, false) => info!("{:?}", config.sanitized()),
    }

    // Init + log public key. Will crash if invalid private key here.
    info!("Public Key: {}", core_keys().load().public);

    rustls::crypto::aws_lc_rs::default_provider()
      .install_default()
      .map_err(|_| {
        anyhow::Error::msg("Failed to install tls crypto provider")
      })?;

    db::init().await?;

    anyhow::Ok(())
  }
  .instrument(startup_span)
  .await?;

  mogh_server::serve_app(api::app(), config).await
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
