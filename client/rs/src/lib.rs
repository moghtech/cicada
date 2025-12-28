pub mod api;
pub mod entities;

mod request;

pub struct CicadaClient {
  #[cfg(not(feature = "blocking"))]
  reqwest: reqwest::Client,
  #[cfg(feature = "blocking")]
  reqwest: reqwest::blocking::Client,
  address: String,
}

impl CicadaClient {
  pub fn new(address: impl Into<String>) -> CicadaClient {
    CicadaClient {
      reqwest: Default::default(),
      address: address.into(),
    }
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
