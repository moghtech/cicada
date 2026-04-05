#[macro_use]
extern crate tracing;

use std::sync::{OnceLock, atomic::AtomicBool};

use anyhow::{Context, anyhow};
use cicada_client::{
  CicadaClient,
  api::{
    read::{GetVersion, filesystem::ListFilesystems},
    write::device::CreateDevice,
  },
  entities::ClientType,
};
use std::path::PathBuf;

use futures_util::{StreamExt as _, stream::FuturesUnordered};
use mogh_pki::Pkcs8PrivateKey;
use tracing::Instrument;

use crate::{
  config::{core_public_key, periphery_config, periphery_keys},
  filesystem::CicadaFs,
};

mod config;
mod filesystem;
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
    tokio::task::spawn_blocking(|| {
      if cicada().read(GetVersion{}).is_ok() {
        return Ok(());
      }

      let Some(onboarding_key) = &config.onboarding_key else {
        return Err(anyhow!("Unable to authenticate with Cicada and no onboarding key is configured."))
      };

      let onboarding_key = Pkcs8PrivateKey::from_maybe_raw_bytes(onboarding_key)?;
      let onboarding_client = CicadaClient::new(
        &periphery_config().core_address,
        ClientType::OnboardingKey,
        &onboarding_key,
        core_public_key(),
      )?;

      onboarding_client.write(
        CreateDevice {
          name: config.connect_as.clone(),
          enabled: true,
          public_key: periphery_keys().load().public.clone().into_inner()
        }
      ).context("Failed to create device")?;

      Ok(())
    })
      .await??;

    let filesystems =
      tokio::task::spawn_blocking(|| cicada().read(ListFilesystems {}))
        .await??;
    let mut handles = FuturesUnordered::new();

    for filesystem in &config.filesystems {
      let (name_or_id, mountpoint) = filesystem
        .split_once(":")
        .map(|(name, path)| (name, config.filesystem_root.join(path)))
        .unwrap_or_else(|| {
          (filesystem.as_str(), config.filesystem_root.join(filesystem))
        });

      let Some(filesystem) = filesystems.iter().find_map(|fs| {
        (fs.id.0.as_bytes() == name_or_id.as_bytes()
          || fs.name == name_or_id)
          .then(|| fs.clone())
      }) else {
        warn!(
          "Did not find filesystem matching '{name_or_id}', skipping..."
        );
        continue;
      };

      if !mountpoint.exists() {
        let _ = std::fs::create_dir_all(&mountpoint);
      }

      handles.push(tokio::task::spawn_blocking(move || {
        info!(
          "Mounting {} ({}) to {mountpoint:?}",
          filesystem.name, filesystem.id.0
        );
        if let Err(e) = CicadaFs::mount(
          filesystem.name.clone(),
          filesystem.id,
          &mountpoint,
        ) {
          error!(
            "Failed to mount filesystem {} to {mountpoint:?} | {e:#}",
            filesystem.name
          )
        }
        if !SHOULD_SHUTDOWN.load(std::sync::atomic::Ordering::SeqCst) {
          warn!(
            "Filesystem {} task has finished unexpectedly",
            filesystem.name
          )
        }
      }));
    }

    // Poll sync tasks for early exit
    while let Some(res) = handles.next().await {
      if let Err(e) = res {
        error!("Task failure: {e:?}");
      }
    }

    warn!("No mounts are active, exiting...");

    Ok(())
  }
  .instrument(startup_span)
  .await
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

  // Collect mountpoints so we can unmount on shutdown.
  let mountpoints: Vec<PathBuf> = config
    .filesystems
    .iter()
    .map(|fs| {
      fs.split_once(":")
        .map(|(_, path)| config.filesystem_root.join(path))
        .unwrap_or_else(|| config.filesystem_root.join(fs))
    })
    .collect();

  let shutdown = async {
    tokio::select! {
      _ = sigterm.recv() => info!("Received SIGTERM, unmounting filesystems..."),
      _ = sigint.recv() => info!("Received SIGINT, unmounting filesystems..."),
    }
  };

  tokio::select! {
    res = tokio::spawn(app()) => res?,
    _ = shutdown => {
      SHOULD_SHUTDOWN.store(true, std::sync::atomic::Ordering::SeqCst);
      unmount::all(&mountpoints);
      Ok(())
    },
  }
}
