use http::{Method, Uri};
use mogh_error::anyhow;
use mogh_pki::{Pkcs8PrivateKey, SpkiPublicKey};

use crate::entities::ClientType;

pub mod api;
pub mod entities;

mod request;

pub struct CicadaClient {
  #[cfg(not(feature = "blocking"))]
  reqwest: reqwest::Client,
  #[cfg(feature = "blocking")]
  reqwest: reqwest::blocking::Client,
  address: String,
  auth_address: String,
  client_type: ClientType,
  /// Raw private key
  private_key: [u8; 32],
  /// Raw public key
  core_public_key: [u8; 32],
}

impl CicadaClient {
  pub fn new(
    address: impl Into<String>,
    client_type: ClientType,
    private_key: &Pkcs8PrivateKey,
    core_public_key: &SpkiPublicKey,
  ) -> anyhow::Result<CicadaClient> {
    let address = address.into();
    let private_key =
      Pkcs8PrivateKey::maybe_raw_bytes(private_key.as_str())?;
    let core_public_key = SpkiPublicKey::maybe_pem_to_raw_bytes(
      core_public_key.as_str(),
    )?;
    Ok(CicadaClient {
      reqwest: Default::default(),
      auth_address: format!("{address}/auth"),
      address,
      client_type,
      private_key,
      core_public_key,
    })
  }

  /// Use a custom reqwest client.
  #[cfg(not(feature = "blocking"))]
  pub fn set_reqwest(mut self, reqwest: reqwest::Client) -> Self {
    self.reqwest = reqwest;
    self
  }

  /// Use a custom reqwest client.
  #[cfg(feature = "blocking")]
  pub fn set_reqwest(
    mut self,
    reqwest: reqwest::blocking::Client,
  ) -> Self {
    self.reqwest = reqwest;
    self
  }
}

pub fn pki_auth_prologue(
  method: &Method,
  uri: &Uri,
  timestamp: i64,
) -> String {
  format!("{method}|{uri}|{timestamp}")
}
