use std::{
  collections::HashSet,
  time::{Duration, UNIX_EPOCH},
};

use anyhow::Context as _;
use cicada_client::{
  api::{
    read::{FindNode, FindNodeWithPath, ListNodes},
    write::{CreateNode, DeleteNode, UpdateNode, UpdateNodeData},
  },
  entities::{
    filesystem::FilesystemId,
    node::{NodeEntity, NodeKind},
  },
};
use fuser::*;

use crate::{cicada, options::FilesystemMountOptions};

fn node_to_file_attr(
  node: NodeEntity,
  uid: u32,
  gid: u32,
) -> FileAttr {
  let size = node
    .data
    .as_ref()
    .map(|data| data.len() as u64)
    .unwrap_or_default();
  let (kind, perm) = match node.kind {
    NodeKind::Folder => {
      (FileType::Directory, node.perm.unwrap_or(0o755))
    }
    NodeKind::File => {
      (FileType::RegularFile, node.perm.unwrap_or(0o644))
    }
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
    uid,
    gid,
    rdev: 0,
    blksize: CicadaFs::BLOCK_SIZE as u32,
    flags: 0,
  }
}

pub struct CicadaFs {
  filesystem: FilesystemId,
  /// For when specific path not provided
  root: FileAttr,
  /// Whether working with filesystem pre or post secret interpolation.
  interpolated: bool,
  /// When non-empty, only these UIDs (plus the mounting user) may access files.
  allowed_uids: HashSet<u32>,
}

impl CicadaFs {
  const TTL: Duration = Duration::from_secs(10);
  const BLOCK_SIZE: u64 = 512;

  pub fn mount(
    FilesystemMountOptions {
      name,
      id,
      path,
      mountpoint,
      rw,
      interpolated,
      uid,
      gid,
    }: FilesystemMountOptions,
    allow_uids: &[u32],
  ) -> anyhow::Result<()> {
    let uid = uid.unwrap_or_else(|| unsafe { libc::getuid() });
    let gid = gid.unwrap_or_else(|| unsafe { libc::getgid() });

    info!("Mounting {mountpoint:?} as {uid}:{gid}");

    let root = if let Some(path) = path
      && path.components().count() > 0
    {
      let node = cicada().read(FindNodeWithPath {
        filesystem: id.clone(),
        path,
        interpolated,
      })?;
      node_to_file_attr(node, uid, gid)
    } else {
      FileAttr {
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
      }
    };

    let mut config = fuser::Config::default();

    config.mount_options = vec![
      MountOption::FSName(name),
      if rw { MountOption::RW } else { MountOption::RO },
    ];
    config.acl = fuser::SessionACL::All;

    let fs = CicadaFs {
      filesystem: id,
      root,
      interpolated,
      allowed_uids: allow_uids.iter().cloned().chain([uid]).collect(),
    };

    // Before mount, make sure any existing mount is cleaned up.
    // This may be possible if cicada periphery did not exit cleanly.
    // Without this, the situation would cause the mount to fail.
    // Failed output is ignored, it is expected to fail because
    // usually the mount won't exist.
    crate::unmount::unmount(&mountpoint, true).ok();

    fuser::mount2(fs, mountpoint, &config)
      .context("Failed to mount CicadaFs")
  }

  fn check_access(&self, req: &fuser::Request) -> bool {
    trace!("GOT CHECK ACCESS FROM UID: {}", req.uid());
    self.allowed_uids.is_empty()
      || self.allowed_uids.contains(&req.uid())
  }

  fn node_to_file_attr(&self, node: NodeEntity) -> FileAttr {
    node_to_file_attr(node, self.root.uid, self.root.gid)
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
      debug!("DENY ACCESS to {}", req.uid());
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
      debug!("DENY READDIR to {}", req.uid());
      reply.error(Errno::EACCES);
      return;
    }
    let ino = if ino == 1 { self.root.ino.0 } else { ino };
    let nodes = match cicada().read(ListNodes {
      filesystem: self.filesystem.clone(),
      parent: ino,
    }) {
      Ok(node) => node,
      Err(e) => {
        debug!(
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
      debug!("DENY LOOKUP to {}", req.uid());
      reply.error(Errno::EACCES);
      return;
    }
    let parent = if parent == 1 { self.root.ino.0 } else { parent };
    let Some(name) = name.to_str() else {
      debug!("LOOKUP FAILED: Name {name:?} is not valid UTF-8");
      reply.error(Errno::ENOENT);
      return;
    };
    let attr = match cicada().read(FindNode::with_parent_name(
      self.filesystem.clone(),
      parent,
      name,
      true,
    )) {
      Ok(node) => self.node_to_file_attr(node),
      Err(e) => {
        debug!(
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
      debug!("DENY GETATTR to {}", req.uid());
      reply.error(Errno::EACCES);
      return;
    }
    let ino = if ino == 1 { self.root.ino.0 } else { ino };
    // handle root case
    if ino == 1 {
      reply.attr(&CicadaFs::TTL, &self.root);
      return;
    }
    let attr = match cicada().read(FindNode::with_inode(
      self.filesystem.clone(),
      ino,
      true,
    )) {
      Ok(node) => self.node_to_file_attr(node),
      Err(e) => {
        debug!(
          "GETATTR FAILED: Could not find node | inode: {ino} | {e:#}"
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
      debug!("DENY READ to {}", req.uid());
      reply.error(Errno::EACCES);
      return;
    }
    let ino = if ino == 1 { self.root.ino.0 } else { ino };
    // Root inode has no data
    if ino == 1 {
      reply.error(Errno::ENOENT);
      return;
    }
    let node = match cicada().read(FindNode::with_inode(
      self.filesystem.clone(),
      ino,
      true,
    )) {
      Ok(node) => node,
      Err(e) => {
        debug!(
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

  fn open(
    &self,
    req: &fuser::Request,
    INodeNo(ino): INodeNo,
    _flags: OpenFlags,
    reply: ReplyOpen,
  ) {
    if !self.check_access(req) {
      debug!("DENY OPEN to {}", req.uid());
      reply.error(Errno::EACCES);
      return;
    }
    let ino = if ino == 1 { self.root.ino.0 } else { ino };
    // Root inode
    if ino == 1 {
      reply.opened(FileHandle(0), FopenFlags::empty());
      return;
    }
    // Verify the node exists
    match cicada().read(FindNode::with_inode(
      self.filesystem.clone(),
      ino,
      true,
    )) {
      Ok(_) => reply.opened(FileHandle(0), FopenFlags::empty()),
      Err(e) => {
        debug!(
          "OPEN FAILED: Could not find node | inode: {ino} | {e:#}"
        );
        reply.error(Errno::ENOENT);
      }
    }
  }

  // =======
  // WRITE
  // =======

  fn create(
    &self,
    req: &fuser::Request,
    INodeNo(parent): INodeNo,
    name: &std::ffi::OsStr,
    mode: u32,
    _umask: u32,
    _flags: i32,
    reply: ReplyCreate,
  ) {
    if !self.check_access(req) {
      debug!("DENY CREATE to {}", req.uid());
      reply.error(Errno::EACCES);
      return;
    }
    let parent = if parent == 1 { self.root.ino.0 } else { parent };
    let Some(name) = name.to_str() else {
      debug!("CREATE FAILED: Name {name:?} is not valid UTF-8");
      reply.error(Errno::EINVAL);
      return;
    };
    let perm = (mode & 0o7777) as u16;
    match cicada().write(CreateNode {
      filesystem: self.filesystem.clone(),
      parent,
      name: name.to_string(),
      perm: Some(perm),
      kind: NodeKind::File,
      data: Some(String::new()),
      interpolated: self.interpolated,
      ..Default::default()
    }) {
      Ok(node) => {
        let attr = self.node_to_file_attr(node);
        reply.created(
          &CicadaFs::TTL,
          &attr,
          Generation(0),
          FileHandle(0),
          FopenFlags::empty(),
        );
      }
      Err(e) => {
        error!(
          "CREATE FAILED: Could not create node | parent: {parent} | name: {name} | {e:#}"
        );
        reply.error(Errno::EIO);
      }
    }
  }

  fn mkdir(
    &self,
    req: &fuser::Request,
    INodeNo(parent): INodeNo,
    name: &std::ffi::OsStr,
    mode: u32,
    _umask: u32,
    reply: fuser::ReplyEntry,
  ) {
    if !self.check_access(req) {
      debug!("DENY MKDIR to {}", req.uid());
      reply.error(Errno::EACCES);
      return;
    }
    let parent = if parent == 1 { self.root.ino.0 } else { parent };
    let Some(name) = name.to_str() else {
      debug!("MKDIR FAILED: Name {name:?} is not valid UTF-8");
      reply.error(Errno::EINVAL);
      return;
    };
    let perm = (mode & 0o7777) as u16;
    match cicada().write(CreateNode {
      filesystem: self.filesystem.clone(),
      parent,
      name: name.to_string(),
      perm: Some(perm),
      kind: NodeKind::Folder,
      interpolated: self.interpolated,
      ..Default::default()
    }) {
      Ok(node) => {
        let attr = self.node_to_file_attr(node);
        reply.entry(&CicadaFs::TTL, &attr, Generation(0));
      }
      Err(e) => {
        error!(
          "MKDIR FAILED: Could not create folder | parent: {parent} | name: {name} | {e:#}"
        );
        reply.error(Errno::EIO);
      }
    }
  }

  fn write(
    &self,
    req: &fuser::Request,
    INodeNo(ino): INodeNo,
    _fh: FileHandle,
    offset: u64,
    data: &[u8],
    _write_flags: WriteFlags,
    _flags: OpenFlags,
    _lock_owner: Option<LockOwner>,
    reply: ReplyWrite,
  ) {
    if !self.check_access(req) {
      debug!("DENY WRITE to {}", req.uid());
      reply.error(Errno::EACCES);
      return;
    }
    let ino = if ino == 1 { self.root.ino.0 } else { ino };
    let node = match cicada().read(FindNode::with_inode(
      self.filesystem.clone(),
      ino,
      true,
    )) {
      Ok(node) => node,
      Err(e) => {
        debug!(
          "WRITE FAILED: Could not find node | inode: {ino} | {e:#}"
        );
        reply.error(Errno::ENOENT);
        return;
      }
    };

    let Ok(new_data) = std::str::from_utf8(data) else {
      debug!("WRITE FAILED: Data is not valid UTF-8 | inode: {ino}");
      reply.error(Errno::EINVAL);
      return;
    };

    // Merge the write into existing data at the given offset
    let mut current = node.data.unwrap_or_default().into_bytes();
    let offset = offset as usize;
    let end = offset + data.len();
    if current.len() < end {
      current.resize(end, 0);
    }
    current[offset..end].copy_from_slice(data);

    let merged = match String::from_utf8(current) {
      Ok(s) => s,
      Err(_) => {
        debug!(
          "WRITE FAILED: Merged data is not valid UTF-8 | inode: {ino}"
        );
        reply.error(Errno::EINVAL);
        return;
      }
    };

    match cicada().write(UpdateNodeData {
      id: node.id,
      data: merged,
      interpolated: self.interpolated,
      ..Default::default()
    }) {
      Ok(_) => reply.written(new_data.len() as u32),
      Err(e) => {
        error!(
          "WRITE FAILED: Could not update node data | inode: {ino} | {e:#}"
        );
        reply.error(Errno::EIO);
      }
    }
  }

  fn setattr(
    &self,
    req: &fuser::Request,
    INodeNo(ino): INodeNo,
    mode: Option<u32>,
    _uid: Option<u32>,
    _gid: Option<u32>,
    size: Option<u64>,
    _atime: Option<fuser::TimeOrNow>,
    _mtime: Option<fuser::TimeOrNow>,
    _ctime: Option<std::time::SystemTime>,
    _fh: Option<FileHandle>,
    _crtime: Option<std::time::SystemTime>,
    _chgtime: Option<std::time::SystemTime>,
    _bkuptime: Option<std::time::SystemTime>,
    _flags: Option<fuser::BsdFileFlags>,
    reply: fuser::ReplyAttr,
  ) {
    if !self.check_access(req) {
      debug!("DENY SETATTR to {}", req.uid());
      reply.error(Errno::EACCES);
      return;
    }
    let ino = if ino == 1 { self.root.ino.0 } else { ino };
    if ino == 1 {
      reply.attr(&CicadaFs::TTL, &self.root);
      return;
    }

    let node = match cicada().read(FindNode::with_inode(
      self.filesystem.clone(),
      ino,
      true,
    )) {
      Ok(node) => node,
      Err(e) => {
        debug!(
          "SETATTR FAILED: Could not find node | inode: {ino} | {e:#}"
        );
        reply.error(Errno::ENOENT);
        return;
      }
    };

    // Handle permission change
    let perm = mode.map(|m| (m & 0o7777) as u16);
    if perm.is_some()
      && let Err(e) = cicada().write(UpdateNode {
        id: node.id.clone(),
        perm,
        ..Default::default()
      })
    {
      error!(
        "SETATTR FAILED: Could not update permissions | inode: {ino} | {e:#}"
      );
      reply.error(Errno::EIO);
      return;
    }

    // Handle truncate
    if let Some(new_size) = size {
      let mut current = node.data.unwrap_or_default().into_bytes();
      current.truncate(new_size as usize);
      let truncated = match String::from_utf8(current) {
        Ok(s) => s,
        Err(_) => {
          debug!(
            "SETATTR FAILED: Truncated data is not valid UTF-8 | inode: {ino}"
          );
          reply.error(Errno::EINVAL);
          return;
        }
      };
      if let Err(e) = cicada().write(UpdateNodeData {
        id: node.id.clone(),
        data: truncated,
        interpolated: self.interpolated,
        ..Default::default()
      }) {
        error!(
          "SETATTR FAILED: Could not truncate node | inode: {ino} | {e:#}"
        );
        reply.error(Errno::EIO);
        return;
      }
    }

    // Re-read updated node for the response
    match cicada().read(FindNode::with_inode(
      self.filesystem.clone(),
      ino,
      true,
    )) {
      Ok(node) => {
        let attr = self.node_to_file_attr(node);
        reply.attr(&CicadaFs::TTL, &attr);
      }
      Err(e) => {
        error!(
          "SETATTR FAILED: Could not re-read node | inode: {ino} | {e:#}"
        );
        reply.error(Errno::EIO);
      }
    }
  }

  // ========
  // DELETE
  // ========

  fn unlink(
    &self,
    req: &fuser::Request,
    INodeNo(parent): INodeNo,
    name: &std::ffi::OsStr,
    reply: fuser::ReplyEmpty,
  ) {
    if !self.check_access(req) {
      debug!("DENY UNLINK to {}", req.uid());
      reply.error(Errno::EACCES);
      return;
    }
    let parent = if parent == 1 { self.root.ino.0 } else { parent };
    let Some(name) = name.to_str() else {
      debug!("UNLINK FAILED: Name {name:?} is not valid UTF-8");
      reply.error(Errno::EINVAL);
      return;
    };
    let node = match cicada().read(FindNode::with_parent_name(
      self.filesystem.clone(),
      parent,
      name,
      true,
    )) {
      Ok(node) => node,
      Err(e) => {
        debug!(
          "UNLINK FAILED: Could not find node | parent: {parent} | name: {name} | {e:#}"
        );
        reply.error(Errno::ENOENT);
        return;
      }
    };
    match cicada().write(DeleteNode {
      id: node.id,
      move_children: None,
      interpolated: self.interpolated,
    }) {
      Ok(_) => reply.ok(),
      Err(e) => {
        error!(
          "UNLINK FAILED: Could not delete node | parent: {parent} | name: {name} | {e:#}"
        );
        reply.error(Errno::EIO);
      }
    }
  }

  fn rmdir(
    &self,
    req: &fuser::Request,
    INodeNo(parent): INodeNo,
    name: &std::ffi::OsStr,
    reply: fuser::ReplyEmpty,
  ) {
    if !self.check_access(req) {
      debug!("DENY RMDIR to {}", req.uid());
      reply.error(Errno::EACCES);
      return;
    }
    let parent = if parent == 1 { self.root.ino.0 } else { parent };
    let Some(name) = name.to_str() else {
      debug!("RMDIR FAILED: Name {name:?} is not valid UTF-8");
      reply.error(Errno::EINVAL);
      return;
    };
    let node = match cicada().read(FindNode::with_parent_name(
      self.filesystem.clone(),
      parent,
      name,
      true,
    )) {
      Ok(node) => node,
      Err(e) => {
        debug!(
          "RMDIR FAILED: Could not find node | parent: {parent} | name: {name} | {e:#}"
        );
        reply.error(Errno::ENOENT);
        return;
      }
    };
    match cicada().write(DeleteNode {
      id: node.id,
      move_children: None,
      interpolated: self.interpolated,
    }) {
      Ok(_) => reply.ok(),
      Err(e) => {
        error!(
          "RMDIR FAILED: Could not delete node | parent: {parent} | name: {name} | {e:#}"
        );
        reply.error(Errno::EIO);
      }
    }
  }

  fn rename(
    &self,
    req: &fuser::Request,
    INodeNo(parent): INodeNo,
    name: &std::ffi::OsStr,
    INodeNo(newparent): INodeNo,
    newname: &std::ffi::OsStr,
    _flags: RenameFlags,
    reply: fuser::ReplyEmpty,
  ) {
    if !self.check_access(req) {
      debug!("DENY RENAME to {}", req.uid());
      reply.error(Errno::EACCES);
      return;
    }
    let parent = if parent == 1 { self.root.ino.0 } else { parent };
    let Some(name) = name.to_str() else {
      debug!("RENAME FAILED: Name {name:?} is not valid UTF-8");
      reply.error(Errno::EINVAL);
      return;
    };
    let Some(newname) = newname.to_str() else {
      debug!(
        "RENAME FAILED: New name {newname:?} is not valid UTF-8"
      );
      reply.error(Errno::EINVAL);
      return;
    };
    let node = match cicada().read(FindNode::with_parent_name(
      self.filesystem.clone(),
      parent,
      name,
      true,
    )) {
      Ok(node) => node,
      Err(e) => {
        debug!(
          "RENAME FAILED: Could not find node | parent: {parent} | name: {name} | {e:#}"
        );
        reply.error(Errno::ENOENT);
        return;
      }
    };
    let new_parent = if newparent != parent {
      Some(newparent)
    } else {
      None
    };
    let new_name = if newname != name {
      Some(newname.to_string())
    } else {
      None
    };
    match cicada().write(UpdateNode {
      id: node.id,
      parent: new_parent,
      name: new_name,
      ..Default::default()
    }) {
      Ok(_) => reply.ok(),
      Err(e) => {
        error!(
          "RENAME FAILED: Could not rename node | parent: {parent} | name: {name} | {e:#}"
        );
        reply.error(Errno::EIO);
      }
    }
  }
}
