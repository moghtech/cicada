use std::{collections::HashMap, path::PathBuf};

use anyhow::Context as _;
use cicada_client::entities::filesystem::{
  FilesystemId, FilesystemRecord,
};

use crate::config::periphery_config;

#[derive(Debug, Clone)]
pub struct FilesystemMountOptions {
  pub name: String,
  pub id: FilesystemId,
  pub node: Option<String>,
  pub mountpoint: PathBuf,
  pub rw: bool,
  pub interpolated: bool,
  pub uid: Option<u32>,
  pub gid: Option<u32>,
}

impl FilesystemMountOptions {
  pub fn parse(
    filesystem_spec: &str,
    filesystems: &[FilesystemRecord],
  ) -> anyhow::Result<FilesystemMountOptions> {
    let filesystem_spec = filesystem_spec.trim();

    let Some((name_or_id, rest)) = filesystem_spec
      .split_once('|')
      .map(|(f, s)| (f.trim(), s.trim()))
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
        node: None,
        mountpoint: periphery_config()
          .default_mount_root
          .join(filesystem_spec),
        rw: false,
        interpolated: true,
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

    let kv_map = rest
      .split('|')
      .flat_map(|kv_pair| {
        let kv_pair = kv_pair.trim();

        let (k, v) = kv_pair.split_once('=')?;

        Some((k.trim(), v.trim()))
      })
      .collect::<HashMap<_, _>>();

    let mountpoint =
      *kv_map.get("mount").unwrap_or(&filesystem.name.as_str());

    let uid = kv_map.get("uid").and_then(|uid| {
      uid
        .parse::<u32>()
        .inspect_err(|e| {
          warn!("Failed to parse uid from {uid} | {e:?}")
        })
        .ok()
    });
    let gid = kv_map
      .get("gid")
      .and_then(|gid| {
        gid
          .parse::<u32>()
          .inspect_err(|e| {
            warn!("Failed to parse gid from {gid} | {e:?}")
          })
          .ok()
      })
      // gid inherits uid if not defined
      .or(uid);

    let rw = *kv_map.get("rw").unwrap_or(&"false") == "true";

    Ok(FilesystemMountOptions {
      name: filesystem.name.clone(),
      id: filesystem.id.clone(),
      node: kv_map
        .get("node")
        .or(kv_map.get("folder"))
        .or(kv_map.get("file"))
        .map(|s| s.to_string()),
      mountpoint: periphery_config()
        .default_mount_root
        .join(mountpoint),
      interpolated: kv_map
        .get("interpolated")
        .map(|v| v == &"true")
        // Otherwise the default is opposite of rw
        .unwrap_or(!rw),
      rw,
      uid,
      gid,
    })
  }
}

// pub struct AllowedUid {
//   pub uid: u32,
//   pub rw: bool,
// }

// impl AllowedUid {
//   pub fn parse(uid_spec: &str) -> anyhow::Result<AllowedUid> {
//     let (uid, rw) = uid_spec
//       .split_once('|')
//       .map(|(uid, rw)| (uid, rw == "rw=true"))
//       .unwrap_or((uid_spec, false));
//     Ok(AllowedUid {
//       uid: uid.parse().context("UID is not valid u32 integer")?,
//       rw,
//     })
//   }
// }
