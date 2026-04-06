use std::path::PathBuf;

use anyhow::Context as _;
use cicada_client::entities::filesystem::{
  FilesystemId, FilesystemRecord,
};

use crate::config::periphery_config;

#[derive(Debug, Clone)]
pub struct FilesystemMountOptions {
  pub name: String,
  pub id: FilesystemId,
  pub mountpoint: PathBuf,
  pub uid: Option<u32>,
  pub gid: Option<u32>,
}

impl FilesystemMountOptions {
  pub fn parse(
    filesystem_spec: &str,
    filesystems: &[FilesystemRecord],
  ) -> anyhow::Result<FilesystemMountOptions> {
    let Some((name_or_id, rest)) = filesystem_spec.split_once(':')
    else {
      let filesystem = filesystems
        .iter()
        .find(|fs| {
          fs.name == filesystem_spec || fs.id.0 == filesystem_spec
        })
        .with_context(|| {
          format!("No filesystem found matching {filesystem_spec}")
        })?;

      return Ok(FilesystemMountOptions {
        name: filesystem.name.clone(),
        id: filesystem.id.clone(),
        mountpoint: periphery_config()
          .default_mount_root
          .join(filesystem_spec),
        uid: None,
        gid: None,
      });
    };

    let filesystem = filesystems
      .iter()
      .find(|fs| fs.name == name_or_id || fs.id.0 == name_or_id)
      .with_context(|| {
        format!("No filesystem found matching {name_or_id}")
      })?;

    // Split UID or GID off end
    let Some((rest, uid_or_gid)) = rest.rsplit_once(':') else {
      // Just uses name / rest
      return Ok(FilesystemMountOptions {
        name: filesystem.name.clone(),
        id: filesystem.id.clone(),
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
      return Ok(FilesystemMountOptions {
        name: filesystem.name.clone(),
        id: filesystem.id.clone(),
        mountpoint: periphery_config().default_mount_root.join(rest),
        uid: Some(uid_or_gid),
        gid: Some(uid_or_gid),
      });
    };

    let uid = uid.parse::<u32>().context("Invalid UID")?;

    Ok(FilesystemMountOptions {
      name: filesystem.name.clone(),
      id: filesystem.id.clone(),
      mountpoint: periphery_config()
        .default_mount_root
        .join(mountpoint),
      uid: Some(uid),
      gid: Some(uid_or_gid),
    })
  }
}
