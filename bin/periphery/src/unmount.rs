use std::{path::Path, time::Duration};

use crate::config::filesystem_mount_options;

pub fn all() {
  std::thread::scope(|s| {
    for options in filesystem_mount_options() {
      s.spawn(|| unmount_with_retries(&options.mountpoint));
    }
  });
}

fn unmount_with_retries(mountpoint: &Path) {
  let mut attempt = 1;
  loop {
    info!("Unmounting {mountpoint:?} (attempt {attempt})");
    match unmount(mountpoint, false) {
      Ok(status) if status.success() => return,
      Ok(status) => {
        error!("fusermount3 exited with {status} for {mountpoint:?}");
        info!("waiting 2s for retry...");
        std::thread::sleep(Duration::from_secs(2));
        attempt += 1;
      }
      Err(e) => {
        error!("[FATAL] Failed to unmount {mountpoint:?} | {e:#}");
        return;
      }
    }
    if attempt > 5 {
      error!(
        "Failed to unmount {mountpoint:?} | Too many retries, exiting anyways"
      );
    }
  }
}

pub fn unmount(
  mountpoint: &Path,
  suppress_output: bool,
) -> std::io::Result<std::process::ExitStatus> {
  let mut child = std::process::Command::new("fusermount3");
  child.arg("-u").arg(mountpoint);
  if suppress_output {
    child
      .stdout(std::process::Stdio::null())
      .stderr(std::process::Stdio::null());
  };
  let mut child = child.spawn()?;
  let timeout = Duration::from_secs(2);
  let start = std::time::Instant::now();
  loop {
    if let Some(status) = child.try_wait()? {
      return Ok(status);
    }
    if start.elapsed() >= timeout {
      let _ = child.kill();
      return Err(std::io::Error::new(
        std::io::ErrorKind::TimedOut,
        "fusermount3 timed out after 2s",
      ));
    }
    std::thread::sleep(Duration::from_millis(50));
  }
}
