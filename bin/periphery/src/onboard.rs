use anyhow::{Context as _, anyhow};
use cicada_client::{
  CicadaClient,
  api::{read::GetVersion, write::CreateDevice},
  entities::ClientType,
};
use mogh_pki::Pkcs8PrivateKey;

use crate::{
  cicada,
  config::{core_public_key, periphery_config, periphery_keys},
};

/// Make sure auth is valid, if not then try onboarding device.
pub async fn ensure_onboarded() -> anyhow::Result<()> {
  tokio::task::spawn_blocking(|| {
    if cicada().read(GetVersion{}).is_ok() {
      info!("Authenticated with Cicada Core");
      return Ok(());
    }

    info!("Failed to authenticate with Cicada Core, attempting onboarding...");

    let config = periphery_config();
    let Some(onboarding_key) = &config.onboarding_key else {
      return Err(anyhow!("Unable to authenticate with Cicada and no onboarding key is configured."))
    };

    let onboarding_key = Pkcs8PrivateKey::from_maybe_raw_bytes(onboarding_key)?;
    let onboarding_client = CicadaClient::new(
      &config.core_address,
      ClientType::OnboardingKey,
      &onboarding_key,
      core_public_key(),
    )?;

    onboarding_client.write(
      CreateDevice {
        name: config.device_name.clone(),
        enabled: true,
        public_key: periphery_keys().load().public.clone().into_inner(),
        groups: Vec::new()
      }
    ).context("Failed to create device")?;

    info!(device_name = config.device_name, "Device onboarded successfully");

    Ok(())
  }).await??;
  Ok(())
}
