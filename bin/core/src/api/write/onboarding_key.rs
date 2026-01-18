use cicada_client::api::write::onboarding_key::*;
use mogh_pki::key::EncodedKeyPair;
use mogh_resolver::Resolve;

use crate::{api::write::WriteArgs, db::query};

#[allow(unused)]
#[utoipa::path(
  post,
  path = "/write/CreateOnboardingKey",
  description = "Create a new onboarding key",
  request_body(content = CreateOnboardingKey),
  responses(
    (status = 200, description = "The created onboarding key", body = CreateOnboardingKeyResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn create_onboarding_key() {}

impl Resolve<WriteArgs> for CreateOnboardingKey {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    let (private_key, public_key) = if let Some(public_key) =
      self.public_key
      && !public_key.is_empty()
    {
      (None, public_key)
    } else {
      let keys = EncodedKeyPair::generate(mogh_pki::PkiType::OneWay)?;
      (Some(keys.private.into_inner()), keys.public.into_inner())
    };
    let created = query::onboarding_key::create_onboarding_key(
      CreateOnboardingKey {
        name: self.name,
        public_key: Some(public_key),
        enabled: self.enabled,
      },
    )
    .await?;
    Ok(CreateOnboardingKeyResponse {
      private_key,
      created,
    })
  }
}

//

#[allow(unused)]
#[utoipa::path(
  post,
  path = "/write/UpdateOnboardingKey",
  description = "Update an onboarding key",
  request_body(content = UpdateOnboardingKey),
  responses(
    (status = 200, description = "The updated onboarding key", body = UpdateOnboardingKeyResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn update_onboarding_key() {}

impl Resolve<WriteArgs> for UpdateOnboardingKey {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::onboarding_key::update_onboarding_key(self).await
  }
}

//

#[allow(unused)]
#[utoipa::path(
  post,
  path = "/write/DeleteOnboardingKey",
  description = "Delete an onboarding key",
  request_body(content = DeleteOnboardingKey),
  responses(
    (status = 200, description = "The deleted onboarding key", body = DeleteOnboardingKeyResponse),
    (status = 404, description = "OnboardingKey not found", body = mogh_error::Serror),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn delete_onboarding_key() {}

impl Resolve<WriteArgs> for DeleteOnboardingKey {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::onboarding_key::delete_onboarding_key(self.id.0).await
  }
}

#[allow(unused)]
#[utoipa::path(
  post,
  path = "/write/BatchDeleteOnboardingKeys",
  description = "Batch delete onboarding keys",
  request_body(content = BatchDeleteOnboardingKeys),
  responses(
    (status = 200, description = "The deleted onboarding keys", body = BatchDeleteOnboardingKeysResponse),
    (status = 500, description = "Request failed", body = mogh_error::Serror)
  ),
)]
pub fn batch_delete_onboarding_keys() {}

impl Resolve<WriteArgs> for BatchDeleteOnboardingKeys {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::onboarding_key::batch_delete_onboarding_keys(self.ids)
      .await
  }
}
