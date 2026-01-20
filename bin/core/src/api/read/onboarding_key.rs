use cicada_client::api::read::onboarding_key::*;
use mogh_resolver::Resolve;

use crate::{api::read::ReadArgs, db::query};

impl Resolve<ReadArgs> for ListOnboardingKeys {
  async fn resolve(
    self,
    _: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::onboarding_key::list_all_onboarding_keys().await
  }
}

//

impl Resolve<ReadArgs> for GetOnboardingKey {
  async fn resolve(
    self,
    _: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::onboarding_key::get_onboarding_key(&self.id.0).await
  }
}
