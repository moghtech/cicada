use http::{Method, Uri};
use mogh_error::anyhow;
use mogh_pki::{Pkcs8PrivateKey, SpkiPublicKey};

use crate::entities::ClientType;

pub mod api;
pub mod entities;

#[cfg(feature = "utoipa")]
pub mod openapi;

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
  #[cfg(not(feature = "blocking"))]
  pub async fn new(
    address: impl Into<String>,
    client_type: ClientType,
    private_key: &Pkcs8PrivateKey,
    // If not provided, will query the given address for it.
    core_public_key: Option<&SpkiPublicKey>,
  ) -> anyhow::Result<CicadaClient> {
    let address = address.into();
    let private_key =
      Pkcs8PrivateKey::maybe_raw_bytes(private_key.as_str())?;
    let mut cicada = CicadaClient {
      reqwest: Default::default(),
      auth_address: format!("{address}/auth"),
      address,
      client_type,
      private_key,
      core_public_key: Default::default(),
    };
    // maybe load public key
    cicada.core_public_key = if let Some(pk) = core_public_key {
      SpkiPublicKey::maybe_pem_to_raw_bytes(pk.as_str())?
    } else {
      SpkiPublicKey::maybe_pem_to_raw_bytes(
        cicada.public_key().await?.as_str(),
      )?
    };
    Ok(cicada)
  }

  #[cfg(feature = "blocking")]
  pub fn new(
    address: impl Into<String>,
    client_type: ClientType,
    private_key: &Pkcs8PrivateKey,
    // If not provided, will query the given address for it.
    core_public_key: Option<&SpkiPublicKey>,
  ) -> anyhow::Result<CicadaClient> {
    let address = address.into();
    let private_key =
      Pkcs8PrivateKey::maybe_raw_bytes(private_key.as_str())?;
    let mut cicada = CicadaClient {
      reqwest: Default::default(),
      auth_address: format!("{address}/auth"),
      address,
      client_type,
      private_key,
      core_public_key: Default::default(),
    };
    // maybe load public key
    cicada.core_public_key = if let Some(pk) = core_public_key {
      SpkiPublicKey::maybe_pem_to_raw_bytes(pk.as_str())?
    } else {
      SpkiPublicKey::maybe_pem_to_raw_bytes(
        cicada.public_key()?.as_str(),
      )?
    };
    Ok(cicada)
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
