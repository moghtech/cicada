use std::{
  path::{Path, PathBuf},
  time::Duration,
};

pub fn all(mountpoints: &[PathBuf]) {
  std::thread::scope(|s| {
    for mountpoint in mountpoints {
      s.spawn(|| unmount_with_retries(mountpoint));
    }
  });
}

fn unmount_with_retries(mountpoint: &Path) {
  for attempt in 1..=3 {
    info!("Unmounting {mountpoint:?} (attempt {attempt}/3)");
    match unmount(mountpoint) {
      Ok(status) if status.success() => return,
      Ok(status) => {
        error!(
          "fusermount3 exited with {status} for {mountpoint:?}"
        );
      }
      Err(e) => {
        error!("Failed to unmount {mountpoint:?} | {e:#}");
      }
    }
  }
}

fn unmount(
  mountpoint: &Path,
) -> std::io::Result<std::process::ExitStatus> {
  let mut child = std::process::Command::new("fusermount3")
    .arg("-u")
    .arg(mountpoint)
    .spawn()?;
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
