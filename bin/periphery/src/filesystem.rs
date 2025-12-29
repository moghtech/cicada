use std::{
  path::Path,
  time::{Duration, UNIX_EPOCH},
};

use anyhow::Context;
use cicada_client::{
  api::read::node::{FindNode, GetNode, ListNodes},
  entities::node::{NodeKind, NodeRecord},
};
use fuser::{FileAttr, FileType, MountOption};
use libc::ENOENT;

use crate::cicada;

pub struct CicadaFs {
  filesystem: String,
  root: FileAttr,
}

impl CicadaFs {
  const TTL: Duration = Duration::from_secs(10);
  const BLOCK_SIZE: u64 = 512;

  pub fn mount<P>(
    filesystem: String,
    mountpoint: P,
  ) -> anyhow::Result<()>
  where
    P: AsRef<Path>,
  {
    let uid = unsafe { libc::getuid() };
    let gid = unsafe { libc::getgid() };
    let root = FileAttr {
      ino: 1,
      size: 0,
      blocks: 0,
      atime: UNIX_EPOCH,
      mtime: UNIX_EPOCH,
      ctime: UNIX_EPOCH,
      crtime: UNIX_EPOCH,
      kind: FileType::Directory,
      perm: 0o755,
      nlink: 2,
      uid,
      gid,
      rdev: 0,
      blksize: CicadaFs::BLOCK_SIZE as u32,
      flags: 0,
    };
    let options = &[
      MountOption::FSName(filesystem.clone()),
      MountOption::RO,
      MountOption::AllowOther,
      MountOption::AutoUnmount,
      MountOption::DefaultPermissions,
    ];
    fuser::mount2(CicadaFs { filesystem, root }, mountpoint, options)
      .context("Failed to mount CicadaFs")
  }

  fn node_to_file_attr(&self, node: NodeRecord) -> FileAttr {
    let size = node
      .data
      .as_ref()
      .map(|data| data.len() as u64)
      .unwrap_or_default();
    let (kind, perm) = match node.kind {
      NodeKind::Folder => (FileType::Directory, 0o700),
      NodeKind::File => (FileType::RegularFile, 0o600),
    };
    FileAttr {
      ino: node.id.0,
      size,
      blocks: size.div_ceil(CicadaFs::BLOCK_SIZE),
      atime: UNIX_EPOCH,
      mtime: UNIX_EPOCH,
      ctime: UNIX_EPOCH,
      crtime: UNIX_EPOCH,
      kind,
      perm,
      nlink: 1,
      uid: self.root.uid,
      gid: self.root.gid,
      rdev: 0,
      blksize: CicadaFs::BLOCK_SIZE as u32,
      flags: 0,
    }
  }
}

impl fuser::Filesystem for CicadaFs {
  // ======
  //  READ
  // ======

  fn readdir(
    &mut self,
    _req: &fuser::Request<'_>,
    ino: u64,
    _fh: u64,
    offset: i64,
    mut reply: fuser::ReplyDirectory,
  ) {
    let nodes = match cicada().read(ListNodes {
      filesystem: self.filesystem.clone(),
      parent: ino,
    }) {
      Ok(node) => node,
      Err(e) => {
        error!(
          "READDIR FAILED: Could not list children nodes | inode: {ino} | {e:#}"
        );
        reply.error(ENOENT);
        return;
      }
    };

    let mut entries = vec![
      (ino, FileType::Directory, "."),
      (ino, FileType::Directory, ".."),
    ];

    entries.extend(nodes.iter().map(|node| {
      let kind = match node.kind {
        NodeKind::Folder => FileType::Directory,
        NodeKind::File => FileType::RegularFile,
      };
      (node.id.0, kind, node.name.as_str())
    }));

    for (i, (ino, kind, name)) in
      entries.into_iter().enumerate().skip(offset as usize)
    {
      // i + 1 means the index of the next entry
      if reply.add(ino, (i + 1) as i64, kind, name) {
        break;
      }
    }

    reply.ok();
  }

  fn lookup(
    &mut self,
    _req: &fuser::Request<'_>,
    parent: u64,
    name: &std::ffi::OsStr,
    reply: fuser::ReplyEntry,
  ) {
    let Some(name) = name.to_str() else {
      error!("LOOKUP FAILED: Name {name:?} is not valid UTF-8");
      reply.error(ENOENT);
      return;
    };
    let attr = match cicada().read(FindNode {
      filesystem: self.filesystem.clone(),
      parent,
      name: String::from(name),
    }) {
      Ok(node) => self.node_to_file_attr(node),
      Err(e) => {
        error!(
          "LOOKUP FAILED: Could not find node | Parent: {parent} | Name: {name} | {e:#}"
        );
        reply.error(ENOENT);
        return;
      }
    };
    reply.entry(&CicadaFs::TTL, &attr, 0);
  }

  fn getattr(
    &mut self,
    _req: &fuser::Request<'_>,
    ino: u64,
    _fh: Option<u64>,
    reply: fuser::ReplyAttr,
  ) {
    // handle root case
    if ino == 1 {
      reply.attr(&CicadaFs::TTL, &self.root);
      return;
    }
    let attr = match cicada().read(GetNode { id: ino }) {
      Ok(node) => self.node_to_file_attr(node),
      Err(e) => {
        error!(
          "LOOKUP FAILED: Could not find node | inode: {ino} | {e:#}"
        );
        reply.error(ENOENT);
        return;
      }
    };
    reply.attr(&CicadaFs::TTL, &attr);
  }

  fn read(
    &mut self,
    _req: &fuser::Request<'_>,
    ino: u64,
    _fh: u64,
    _offset: i64,
    _size: u32,
    _flags: i32,
    _lock_owner: Option<u64>,
    reply: fuser::ReplyData,
  ) {
    // Root inode has no data
    if ino == 1 {
      reply.error(ENOENT);
      return;
    }
    let node = match cicada().read(GetNode { id: ino }) {
      Ok(node) => node,
      Err(e) => {
        error!(
          "READ FAILED: Could not find node | inode: {ino} | {e:#}"
        );
        reply.error(ENOENT);
        return;
      }
    };
    match node.data {
      Some(data) => {
        reply.data(data.as_bytes());
      }
      None => {
        error!("READ FAILED: No data found for node | inode: {ino}");
        reply.error(ENOENT);
      }
    }
  }
}
