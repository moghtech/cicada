use cicada_client::api::write::onboarding_key::*;
use mogh_pki::EncodedKeyPair;
use mogh_resolver::Resolve;

use crate::{api::write::WriteArgs, db::query};

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
      let keys = EncodedKeyPair::generate(mogh_pki::PkiKind::OneWay)?;
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

impl Resolve<WriteArgs> for UpdateOnboardingKey {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::onboarding_key::update_onboarding_key(self).await
  }
}

//

impl Resolve<WriteArgs> for DeleteOnboardingKey {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::onboarding_key::delete_onboarding_key(self.id.0).await
  }
}

impl Resolve<WriteArgs> for BatchDeleteOnboardingKeys {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::onboarding_key::batch_delete_onboarding_keys(self.ids)
      .await
  }
}
