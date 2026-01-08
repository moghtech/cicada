use anyhow::{Context, anyhow};
use mogh_auth_client::api::login::MoghAuthLoginRequest;
#[cfg(not(feature = "blocking"))]
use mogh_auth_client::api::manage::MoghAuthManageRequest;
use mogh_error::deserialize_error;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::json;

use crate::{
  CicadaClient,
  api::{read::CicadaReadRequest, write::CicadaWriteRequest},
};

impl CicadaClient {
  #[cfg(not(feature = "blocking"))]
  pub async fn auth_login<T>(
    &self,
    request: T,
  ) -> anyhow::Result<T::Response>
  where
    T: Serialize + MoghAuthLoginRequest,
    T::Response: DeserializeOwned,
  {
    mogh_auth_client::request::login(
      &self.reqwest,
      &self.auth_address,
      request,
    )
    .await
  }

  #[cfg(feature = "blocking")]
  pub fn auth_login<T>(
    &self,
    request: T,
  ) -> anyhow::Result<T::Response>
  where
    T: Serialize + MoghAuthLoginRequest,
    T::Response: DeserializeOwned,
  {
    mogh_auth_client::request::login(
      &self.reqwest,
      &self.auth_address,
      request,
    )
  }

  #[cfg(not(feature = "blocking"))]
  pub async fn auth_manage<T>(
    &self,
    request: T,
  ) -> anyhow::Result<T::Response>
  where
    T: Serialize + MoghAuthManageRequest,
    T::Response: DeserializeOwned,
  {
    mogh_auth_client::request::manage(
      &self.reqwest,
      &self.auth_address,
      request,
    )
    .await
  }

  #[cfg(feature = "blocking")]
  pub fn auth_manage<T>(
    &self,
    request: T,
  ) -> anyhow::Result<T::Response>
  where
    T: Serialize + MoghAuthManageRequest,
    T::Response: DeserializeOwned,
  {
    mogh_auth_client::request::manage(
      &self.reqwest,
      &self.auth_address,
      request,
    )
  }

  #[cfg(not(feature = "blocking"))]
  pub async fn read<T>(
    &self,
    request: T,
  ) -> anyhow::Result<T::Response>
  where
    T: Serialize + CicadaReadRequest,
    T::Response: DeserializeOwned,
  {
    self
      .post(
        "/read",
        json!({
          "type": T::req_type(),
          "params": request
        }),
      )
      .await
  }

  #[cfg(feature = "blocking")]
  pub fn read<T>(&self, request: T) -> anyhow::Result<T::Response>
  where
    T: Serialize + CicadaReadRequest,
    T::Response: DeserializeOwned,
  {
    self.post(
      "/read",
      json!({
        "type": T::req_type(),
        "params": request
      }),
    )
  }

  #[cfg(not(feature = "blocking"))]
  pub async fn write<T>(
    &self,
    request: T,
  ) -> anyhow::Result<T::Response>
  where
    T: Serialize + CicadaWriteRequest,
    T::Response: DeserializeOwned,
  {
    self
      .post(
        "/write",
        json!({
          "type": T::req_type(),
          "params": request
        }),
      )
      .await
  }

  #[cfg(feature = "blocking")]
  pub fn write<T>(&self, request: T) -> anyhow::Result<T::Response>
  where
    T: Serialize + CicadaWriteRequest,
    T::Response: DeserializeOwned,
  {
    self.post(
      "/write",
      json!({
        "type": T::req_type(),
        "params": request
      }),
    )
  }

  #[cfg(not(feature = "blocking"))]
  async fn post<
    B: Serialize + std::fmt::Debug,
    R: DeserializeOwned,
  >(
    &self,
    endpoint: &str,
    body: B,
  ) -> anyhow::Result<R> {
    let req = self
      .reqwest
      .post(format!("{}{endpoint}", self.address))
      // .header("x-api-key", &self.key)
      // .header("x-api-secret", &self.secret)
      .header("content-type", "application/json")
      .json(&body);
    let res =
      req.send().await.context("failed to reach Cicada API")?;
    let status = res.status();
    if status.is_success() {
      match res.json().await {
        Ok(res) => Ok(res),
        Err(e) => Err(anyhow!("{e:#?}").context(status)),
      }
    } else {
      match res.text().await {
        Ok(res) => Err(deserialize_error(res).context(status)),
        Err(e) => Err(anyhow!("{e:?}").context(status)),
      }
    }
  }

  #[cfg(feature = "blocking")]
  fn post<B: Serialize + std::fmt::Debug, R: DeserializeOwned>(
    &self,
    endpoint: &str,
    body: B,
  ) -> anyhow::Result<R> {
    let req = self
      .reqwest
      .post(format!("{}{endpoint}", self.address))
      // .header("x-api-key", &self.key)
      // .header("x-api-secret", &self.secret)
      .header("content-type", "application/json")
      .json(&body);
    let res = req.send().context("failed to reach Cicada API")?;
    let status = res.status();
    if status.is_success() {
      match res.json() {
        Ok(res) => Ok(res),
        Err(e) => Err(anyhow!("{e:#?}").context(status)),
      }
    } else {
      match res.text() {
        Ok(res) => Err(deserialize_error(res).context(status)),
        Err(e) => Err(anyhow!("{e:?}").context(status)),
      }
    }
  }
}
