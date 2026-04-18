#[macro_use]
extern crate tracing;

use std::{
  sync::{OnceLock, atomic::AtomicBool},
  time::Duration,
};

use cicada_client::{CicadaClient, entities::ClientType};
use tracing::Instrument;

use crate::config::{
  core_public_key, periphery_config, periphery_keys,
};

mod config;
mod filesystem;
mod mount;
mod onboard;
mod options;
mod unmount;

fn cicada() -> &'static CicadaClient {
  static CICADA: OnceLock<CicadaClient> = OnceLock::new();
  CICADA.get_or_init(|| {
    CicadaClient::new(
      &periphery_config().core_address,
      ClientType::Device,
      &periphery_keys().load().private,
      core_public_key(),
    )
    .unwrap()
  })
}

static SHOULD_SHUTDOWN: AtomicBool = AtomicBool::new(false);

async fn app() -> anyhow::Result<()> {
  let startup_span = info_span!("PeripheryStartup");

  async {
    info!("Cicada Periphery version: v{}", env!("CARGO_PKG_VERSION"));

    let config = periphery_config();

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
    info!("Public Key: {}", periphery_keys().load().public);

    // Make sure auth is valid, if not then try onboarding device.
    onboard::ensure_onboarded().await?;

    anyhow::Ok(())
  }
  .instrument(startup_span)
  .await?;

  // Mount the configured filesystems
  mount::filesystems().await
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  dotenvy::dotenv().ok();
  let config = periphery_config();

  mogh_logger::init(&config.logging)?;

  let mut sigterm = tokio::signal::unix::signal(
    tokio::signal::unix::SignalKind::terminate(),
  )?;
  let mut sigint = tokio::signal::unix::signal(
    tokio::signal::unix::SignalKind::interrupt(),
  )?;

  let shutdown = async {
    tokio::select! {
      _ = sigterm.recv() => warn!("Received SIGTERM, unmounting filesystems..."),
      _ = sigint.recv() => warn!("Received SIGINT, unmounting filesystems..."),
    }
  };

  let mut app = tokio::spawn(app());

  tokio::select! {
    res = &mut app => res?,
    _ = shutdown => {
      SHOULD_SHUTDOWN.store(true, std::sync::atomic::Ordering::SeqCst);
      unmount::all();
      tokio::time::timeout(Duration::from_secs(1), app).await???;
      Ok(())
    },
  }
}
