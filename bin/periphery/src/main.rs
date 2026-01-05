#[macro_use]
extern crate tracing;

use std::sync::OnceLock;

use cicada_client::{
  CicadaClient, api::read::filesystem::ListFilesystems,
};
use futures_util::{StreamExt as _, stream::FuturesUnordered};

use crate::{config::periphery_config, filesystem::CicadaFs};

mod config;
mod filesystem;

fn cicada() -> &'static CicadaClient {
  static CICADA: OnceLock<CicadaClient> = OnceLock::new();
  CICADA.get_or_init(|| {
    CicadaClient::new(&periphery_config().core_address)
  })
}

async fn app() -> anyhow::Result<()> {
  dotenvy::dotenv().ok();
  let config = periphery_config();
  mogh_logger::init(&config.logging)?;

  info!("Cicada Periphery version: v{}", env!("CARGO_PKG_VERSION"));
  info!("{config:?}");

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
      warn!(
        "Filesystem {} task has finished unexpectedly",
        filesystem.name
      )
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
