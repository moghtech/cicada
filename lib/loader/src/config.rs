use std::{path::PathBuf, sync::OnceLock};

use mogh_pki::{RotatableKeyPair, SpkiPublicKey};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Env {
  /// Address of Cicada Core
  pub cicada_core_address: String,
  /// Allow Periphery to connect to Core
  /// without validating the Core certs
  #[serde(default)]
  pub cicada_core_tls_insecure_skip_verify: bool,
  /// Specify the core public key to use with authentication signature.
  /// If not specified, will be retreived from the Core '/public_key' route.
  #[serde(default)]
  pub cicada_core_public_key: String,
  /// Private key to use with Noise handshake to authenticate with Cicada Core.
  ///
  /// Supports openssl generated pem file, `openssl genpkey -algorithm X25519 -out private.key`.
  /// To load from file, use `private_key = "file:/path/to/private.key"`.
  ///
  /// If a file is specified and does not exist, will try to generate one at the path
  /// and use it going forward.
  ///
  /// Default: file:/config/keys/cperiphery.key
  #[serde(default = "default_private_key")]
  pub cicada_private_key: String,
  /// Provide an onboarding key to use with the new Device
  /// creation flow.
  pub cicada_onboarding_key: Option<String>,
  /// Set `CICADA_ONBOARDING_KEY` with file
  pub cicada_onboarding_key_file: Option<PathBuf>,
  /// The device name to onboard as.
  /// Note. This name is only used during onboarding.
  /// Every device needs a unique name paired with public key.
  pub cicada_device_name: Option<String>,
}

fn default_private_key() -> String {
  String::from("file:/config/keys/cperiphery.key")
}

pub(crate) fn cperiphery_keys(
  spec: &str,
) -> &'static RotatableKeyPair {
  static PERIPHERY_KEYS: OnceLock<RotatableKeyPair> = OnceLock::new();
  PERIPHERY_KEYS.get_or_init(|| {
    RotatableKeyPair::from_private_key_spec(
      mogh_pki::PkiKind::OneWay,
      spec,
    )
    .unwrap()
  })
}

pub(crate) fn core_public_key(
  core_public_key: &str,
) -> Option<&'static SpkiPublicKey> {
  static CORE_PUBLIC_KEY: OnceLock<Option<SpkiPublicKey>> =
    OnceLock::new();
  CORE_PUBLIC_KEY
    .get_or_init(|| {
      if core_public_key.is_empty() {
        return None;
      }
      Some(SpkiPublicKey::from_spec(core_public_key).unwrap())
    })
    .as_ref()
}
