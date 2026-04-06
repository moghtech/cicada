use cicada_client::api::read::filesystem::ListFilesystems;
use futures_util::{StreamExt as _, stream::FuturesUnordered};

use crate::{cicada, config::periphery_config, filesystem::CicadaFs};

pub async fn filesystems() -> anyhow::Result<()> {
  let filesystems =
    tokio::task::spawn_blocking(|| cicada().read(ListFilesystems {}))
      .await??;
  let mut handles = FuturesUnordered::new();

  let config = periphery_config();

  for filesystem in &config.filesystems {
    let (name_or_id, mountpoint) = filesystem
      .split_once(":")
      .map(|(name, path)| {
        (name, config.default_mount_root.join(path))
      })
      .unwrap_or_else(|| {
        (
          filesystem.as_str(),
          config.default_mount_root.join(filesystem),
        )
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

    let allow_uids = config.allow_uids.clone();
    handles.push(tokio::task::spawn_blocking(move || {
      info!(
        "Mounting {} ({}) to {mountpoint:?}",
        filesystem.name, filesystem.id.0
      );
      if let Err(e) = CicadaFs::mount(
        filesystem.name.clone(),
        filesystem.id,
        &mountpoint,
        allow_uids,
      ) {
        error!(
          "Failed to mount filesystem {} to {mountpoint:?} | {e:#}",
          filesystem.name
        )
      }
      if !crate::SHOULD_SHUTDOWN
        .load(std::sync::atomic::Ordering::SeqCst)
      {
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

  info!("No mounts are active, exiting...");

  Ok(())
}
