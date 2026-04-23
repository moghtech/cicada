use std::{
  path::{Path, PathBuf},
  sync::OnceLock,
};

use anyhow::{Context as _, anyhow};
use cicada_client::{
  CicadaClient,
  api::{
    read::{FindNodeWithPath, GetFilesystem, GetVersion},
    write::CreateDevice,
  },
  entities::ClientType,
};
use mogh_pki::Pkcs8PrivateKey;
use tracing::info;

pub mod config;

use crate::config::{Env, core_public_key, cperiphery_keys};

static CICADA: OnceLock<CicadaClient> = OnceLock::new();

/// Loads the configuration at path from Cicada.
///
/// If needed, will perform one time initialization of cicada client using environment vars.
/// This includes health check, key generation, and device onboarding when necessary.
///
/// Ensure this method is **not** called more than once concurrently.
/// Async runtimes like tokio must run this in blocking task / thread.
pub fn load(path: &Path) -> anyhow::Result<String> {
  let cicada = cicada()?;

  let mut components = path.components();

  let filesystem = components
    .next()
    .context("Missing filesystem components, expecting name or id")?
    .as_os_str()
    .to_str()
    .context("Filesystem is not valid unicode")?
    .to_string();

  let filesystem = cicada.read(GetFilesystem { id: filesystem })?;

  // The rest of the components are the path under filesystem
  let path = components.collect::<PathBuf>();

  cicada
    .read(FindNodeWithPath {
      filesystem: filesystem.id,
      path,
      interpolated: true,
    })?
    .data
    .context("Node has no data available")
}

/// Perform one time initialization of cicada client using environment vars.
/// This includes health check, key generation, and device onboarding when necessary.
///
/// Ensure this method is **not** called more than once concurrently.
pub fn cicada() -> anyhow::Result<&'static CicadaClient> {
  // Early return if already init.
  // Just make sure this fn isn't called multiple times concurrently
  if let Some(cicada) = CICADA.get() {
    return Ok(cicada);
  }

  let env = envy::from_env::<Env>()?;

  let cicada = CicadaClient::new(
    env.cicada_core_address.clone(),
    ClientType::Device,
    &cperiphery_keys(&env.cicada_private_key).load().private,
    core_public_key(&env.cicada_core_public_key),
  )?;

  ensure_onboarded(&cicada, &env)?;

  CICADA
    .set(cicada)
    .map_err(|_| anyhow!("CICADA client lock already initialized"))?;

  CICADA
    .get()
    .context("CICADA client lock missing after initialization")
}

fn ensure_onboarded(
  cicada: &CicadaClient,
  env: &Env,
) -> anyhow::Result<()> {
  if cicada.read(GetVersion {}).is_ok() {
    info!("Authenticated with Cicada Core");
    return Ok(());
  }

  info!(
    "Failed to authenticate with Cicada Core, attempting onboarding..."
  );

  let Some(onboarding_key) = &env.cicada_onboarding_key else {
    return Err(anyhow!(
      "Unable to authenticate with Cicada and no CICADA_ONBOARDING_KEY is configured."
    ));
  };

  let Some(device_name) = &env.cicada_device_name else {
    return Err(anyhow!(
      "Unable to authenticate with Cicada and no CICADA_DEVICE_NAME is configured."
    ));
  };

  let onboarding_key =
    Pkcs8PrivateKey::from_maybe_raw_bytes(onboarding_key)?;
  let onboarding_client = CicadaClient::new(
    &env.cicada_core_address,
    ClientType::OnboardingKey,
    &onboarding_key,
    core_public_key(&env.cicada_core_public_key),
  )?;

  onboarding_client
    .write(CreateDevice {
      name: device_name.to_string(),
      enabled: true,
      public_key: cperiphery_keys(&env.cicada_private_key)
        .load()
        .public
        .clone()
        .into_inner(),
      groups: Vec::new(),
    })
    .context("Failed to create device")?;

  info!(
    device_name = env.cicada_device_name,
    "Device onboarded successfully"
  );

  Ok(())
}
