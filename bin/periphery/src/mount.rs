use futures_util::{StreamExt as _, stream::FuturesUnordered};

use crate::{
  config::{filesystem_mount_options, periphery_config},
  filesystem::CicadaFs,
};

pub async fn filesystems() -> anyhow::Result<()> {
  let mut handles = FuturesUnordered::new();

  let config = periphery_config();

  for options in filesystem_mount_options() {
    if !options.mountpoint.exists() {
      let _ = std::fs::create_dir_all(&options.mountpoint);
    }

    let allow_uids = config.allow_uids.clone();
    handles.push(tokio::task::spawn_blocking(move || {
      info!(
        "Mounting {} ({}) to {:?}",
        options.name, options.id.0, options.mountpoint,
      );
      if let Err(e) = CicadaFs::mount(options.clone(), allow_uids) {
        error!("Failed to mount filesystem {} | {e:#}", options.name)
      }
      if !crate::SHOULD_SHUTDOWN
        .load(std::sync::atomic::Ordering::SeqCst)
      {
        warn!(
          "Filesystem {} task has finished unexpectedly",
          options.name
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
