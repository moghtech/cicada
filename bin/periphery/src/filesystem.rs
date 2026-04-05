use std::{
  collections::HashSet,
  path::Path,
  time::{Duration, UNIX_EPOCH},
};

use anyhow::Context as _;
use cicada_client::{
  api::read::node::{FindNode, ListNodes},
  entities::{
    filesystem::FilesystemId,
    node::{NodeEntity, NodeKind},
  },
};
use fuser::{
  Errno, FileAttr, FileHandle, FileType, Generation, INodeNo,
  LockOwner, MountOption, OpenFlags,
};

use crate::cicada;

pub struct CicadaFs {
  filesystem: FilesystemId,
  root: FileAttr,
  /// When non-empty, only these UIDs (plus the mounting user) may access files.
  allowed_uids: HashSet<u32>,
}

impl CicadaFs {
  const TTL: Duration = Duration::from_secs(10);
  const BLOCK_SIZE: u64 = 512;

  pub fn mount<P>(
    name: String,
    filesystem: FilesystemId,
    mountpoint: P,
    allow_uids: Vec<u32>,
  ) -> anyhow::Result<()>
  where
    P: AsRef<Path>,
  {
    let (uid, gid) = if let Some(uid) = allow_uids.first() {
      (*uid, *uid)
    } else {
      (unsafe { libc::getuid() }, unsafe { libc::getgid() })
    };
    let root = FileAttr {
      ino: INodeNo::ROOT,
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
    let mut options =
      vec![MountOption::FSName(name), MountOption::RO];
    let mut allowed_uids = HashSet::new();
    if !allow_uids.is_empty() {
      // allow_other lets other UIDs reach the filesystem,
      // then we check req.uid() ourselves in each handler.
      options.push(MountOption::CUSTOM("allow_other".into()));
      allowed_uids.insert(uid);
      allowed_uids.extend(allow_uids);
    } else {
      // No extra UIDs — only the mounting user can access,
      // let the kernel handle permission checks.
      options.push(MountOption::DefaultPermissions);
    }
    let mut config = fuser::Config::default();
    config.mount_options = options;
    let fs = CicadaFs {
      filesystem,
      root,
      allowed_uids,
    };
    fuser::mount2(fs, mountpoint, &config)
      .context("Failed to mount CicadaFs")
  }

  fn check_access(&self, req: &fuser::Request) -> bool {
    trace!("GOT CHECK ACCESS FROM UID: {}", req.uid());
    self.allowed_uids.is_empty()
      || self.allowed_uids.contains(&req.uid())
  }

  fn node_to_file_attr(&self, node: NodeEntity) -> FileAttr {
    let size = node
      .data
      .as_ref()
      .map(|data| data.len() as u64)
      .unwrap_or_default();
    let (kind, perm) = match node.kind {
      NodeKind::Folder => (FileType::Directory, 0o755),
      NodeKind::File => (FileType::RegularFile, 0o644),
    };
    FileAttr {
      ino: INodeNo(node.inode),
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
  fn access(
    &self,
    req: &fuser::Request,
    _ino: INodeNo,
    _mask: fuser::AccessFlags,
    reply: fuser::ReplyEmpty,
  ) {
    trace!("GOT ACCESS FROM UID: {}", req.uid());
    if self.check_access(req) {
      reply.ok();
    } else {
      reply.error(Errno::EACCES);
    }
  }

  // ======
  //  READ
  // ======

  fn readdir(
    &self,
    req: &fuser::Request,
    INodeNo(ino): INodeNo,
    _fh: FileHandle,
    offset: u64,
    mut reply: fuser::ReplyDirectory,
  ) {
    if !self.check_access(req) {
      reply.error(Errno::EACCES);
      return;
    }
    let nodes = match cicada().read(ListNodes {
      filesystem: self.filesystem.clone().into(),
      parent: ino.into(),
    }) {
      Ok(node) => node,
      Err(e) => {
        error!(
          "READDIR FAILED: Could not list children nodes | inode: {ino} | {e:#}"
        );
        reply.error(Errno::ENOENT);
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
      (node.inode, kind, node.name.as_str())
    }));

    for (i, (ino, kind, name)) in
      entries.into_iter().enumerate().skip(offset as usize)
    {
      // i + 1 means the index of the next entry
      if reply.add(INodeNo(ino), (i + 1) as u64, kind, name) {
        break;
      }
    }

    reply.ok();
  }

  fn lookup(
    &self,
    req: &fuser::Request,
    INodeNo(parent): INodeNo,
    name: &std::ffi::OsStr,
    reply: fuser::ReplyEntry,
  ) {
    if !self.check_access(req) {
      reply.error(Errno::EACCES);
      return;
    }
    let Some(name) = name.to_str() else {
      error!("LOOKUP FAILED: Name {name:?} is not valid UTF-8");
      reply.error(Errno::ENOENT);
      return;
    };
    let attr = match cicada().read(FindNode::with_parent_name(
      self.filesystem.clone(),
      parent,
      name,
    )) {
      Ok(node) => self.node_to_file_attr(node),
      Err(e) => {
        error!(
          "LOOKUP FAILED: Could not find node | Parent: {parent} | Name: {name} | {e:#}"
        );
        reply.error(Errno::ENOENT);
        return;
      }
    };
    reply.entry(&CicadaFs::TTL, &attr, Generation(0));
  }

  fn getattr(
    &self,
    req: &fuser::Request,
    INodeNo(ino): INodeNo,
    _fh: Option<FileHandle>,
    reply: fuser::ReplyAttr,
  ) {
    if !self.check_access(req) {
      reply.error(Errno::EACCES);
      return;
    }
    // handle root case
    if ino == 1 {
      reply.attr(&CicadaFs::TTL, &self.root);
      return;
    }
    let attr = match cicada()
      .read(FindNode::with_inode(self.filesystem.clone(), ino))
    {
      Ok(node) => self.node_to_file_attr(node),
      Err(e) => {
        error!(
          "LOOKUP FAILED: Could not find node | inode: {ino} | {e:#}"
        );
        reply.error(Errno::ENOENT);
        return;
      }
    };
    reply.attr(&CicadaFs::TTL, &attr);
  }

  fn read(
    &self,
    req: &fuser::Request,
    INodeNo(ino): INodeNo,
    _fh: FileHandle,
    _offset: u64,
    _size: u32,
    _flags: OpenFlags,
    _lock_owner: Option<LockOwner>,
    reply: fuser::ReplyData,
  ) {
    if !self.check_access(req) {
      reply.error(Errno::EACCES);
      return;
    }
    // Root inode has no data
    if ino == 1 {
      reply.error(Errno::ENOENT);
      return;
    }
    let node = match cicada()
      .read(FindNode::with_inode(self.filesystem.clone(), ino))
    {
      Ok(node) => node,
      Err(e) => {
        error!(
          "READ FAILED: Could not find node | inode: {ino} | {e:#}"
        );
        reply.error(Errno::ENOENT);
        return;
      }
    };
    match node.data {
      Some(data) => {
        reply.data(data.as_bytes());
      }
      None => {
        error!("READ FAILED: No data found for node | inode: {ino}");
        reply.error(Errno::ENOENT);
      }
    }
  }
}
