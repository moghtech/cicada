use cicada_client::{
  api::write::onboarding_key::*, entities::Iso8601Timestamp,
};
use mogh_auth_server::rand::random_string;
use mogh_pki::EncodedKeyPair;
use mogh_resolver::Resolve;

use crate::{
  api::write::WriteArgs,
  db::query::{self, onboarding_key::CreateOnboardingKeyQuery},
};

impl Resolve<WriteArgs> for CreateOnboardingKey {
  async fn resolve(
    self,
    _: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    let private_key = if let Some(private_key) = self.private_key
      && !private_key.trim().is_empty()
    {
      private_key
    } else {
      format!("O_{}_O", random_string(28))
    };

    let public_key = EncodedKeyPair::from_private_key(
      mogh_pki::PkiKind::OneWay,
      &private_key,
    )?
    .public
    .into_inner();

    let expires = if self.expires == 0 {
      None
    } else {
      Iso8601Timestamp::from_timestamp(self.expires as i64 / 1_000, 0)
    };

    let created = query::onboarding_key::create_onboarding_key(
      CreateOnboardingKeyQuery {
        name: self.name,
        enabled: self.enabled,
        public_key,
        expires,
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
