use anyhow::Context;
use cicada_client::api::read::filesystem::ListFilesystems;
use futures_util::{StreamExt as _, stream::FuturesUnordered};

use crate::{
  cicada,
  config::periphery_config,
  filesystem::{CicadaFs, MountOptions},
};

pub async fn filesystems() -> anyhow::Result<()> {
  let filesystems =
    tokio::task::spawn_blocking(|| cicada().read(ListFilesystems {}))
      .await??;
  let mut handles = FuturesUnordered::new();

  let config = periphery_config();

  for filesystem in &config.filesystems {
    let mut options = match parse_options(filesystem) {
      Ok(options) => options,
      Err(e) => {
        error!(
          "Failed to parse options from spec: '{filesystem}' | {e:#}"
        );
        continue;
      }
    };

    let Some(filesystem) = filesystems.iter().find_map(|fs| {
      (fs.id.0.as_bytes() == options.name.as_bytes()
        || fs.name == options.name)
        .then(|| fs.clone())
    }) else {
      warn!(
        "Did not find filesystem matching '{}', skipping...",
        options.name
      );
      continue;
    };

    // Set name if it was passed as id
    if filesystem.name != options.name {
      options.name = filesystem.name.clone();
    }

    if !options.mountpoint.exists() {
      let _ = std::fs::create_dir_all(&options.mountpoint);
    }

    let allow_uids = config.allow_uids.clone();
    handles.push(tokio::task::spawn_blocking(move || {
      info!(
        "Mounting {} ({}) to {:?}",
        filesystem.name, filesystem.id.0, options.mountpoint,
      );
      if let Err(e) =
        CicadaFs::mount(filesystem.id, options, allow_uids)
      {
        error!(
          "Failed to mount filesystem {} | {e:#}",
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

fn parse_options(spec: &str) -> anyhow::Result<MountOptions> {
  let Some((name, rest)) = spec.split_once(':') else {
    return Ok(MountOptions {
      name: spec.to_string(),
      mountpoint: periphery_config().default_mount_root.join(spec),
      uid: None,
      gid: None,
    });
  };

  // Split UID or GID off end
  let Some((rest, uid_or_gid)) = rest.rsplit_once(':') else {
    // Just uses name / rest
    return Ok(MountOptions {
      name: name.to_string(),
      mountpoint: periphery_config().default_mount_root.join(rest),
      uid: None,
      gid: None,
    });
  };

  let uid_or_gid =
    uid_or_gid.parse::<u32>().context("Invalid UID/GID")?;

  // Split UID off end
  let Some((mountpoint, uid)) = rest.rsplit_once(':') else {
    // name / rest / uid
    return Ok(MountOptions {
      name: name.to_string(),
      mountpoint: periphery_config().default_mount_root.join(rest),
      uid: Some(uid_or_gid),
      gid: Some(uid_or_gid),
    });
  };

  let uid = uid.parse::<u32>().context("Invalid UID")?;

  Ok(MountOptions {
    name: name.to_string(),
    mountpoint: periphery_config()
      .default_mount_root
      .join(mountpoint),
    uid: Some(uid),
    gid: Some(uid_or_gid),
  })
}
