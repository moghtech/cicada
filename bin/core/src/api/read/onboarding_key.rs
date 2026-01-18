use cicada_client::api::read::onboarding_key::*;
use mogh_resolver::Resolve;

use crate::{api::read::ReadArgs, db::query};

#[allow(unused)]
#[utoipa::path(
  post,
  path = "/read/ListOnboardingKeys",
  description = "List onboarding keys",
  request_body(content = ListOnboardingKeys),
  responses(
    (status = 200, description = "List of onboarding keys", body = ListOnboardingKeysResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn list_onboarding_keys() {}

impl Resolve<ReadArgs> for ListOnboardingKeys {
  async fn resolve(
    self,
    _: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::onboarding_key::list_all_onboarding_keys().await
  }
}

//

#[allow(unused)]
#[utoipa::path(
  post,
  path = "/read/GetOnboardingKey",
  description = "Get an onboarding key by id",
  request_body(content = GetOnboardingKey),
  responses(
    (status = 200, description = "The onboarding key", body = GetOnboardingKeyResponse),
    (status = 404, description = "Failed to find onboarding key with given id", body = mogh_error::Serror),
    (status = 500, description = "Request failed", body = mogh_error::Serror),
  ),
)]
pub fn get_onboarding_key(body: GetOnboardingKey) {}

impl Resolve<ReadArgs> for GetOnboardingKey {
  async fn resolve(
    self,
    _: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::onboarding_key::get_onboarding_key(&self.id.0).await
  }
}
