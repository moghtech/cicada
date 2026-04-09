use cicada_client::api::read::{FindSecret, GetSecret, ListSecrets};
use mogh_resolver::Resolve;

use crate::{
  api::read::ReadArgs, db::query, encryption::decrypt_secret,
};

impl Resolve<ReadArgs> for ListSecrets {
  async fn resolve(
    self,
    ReadArgs { client: _client }: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    query::secret::list_secrets().await
  }
}

impl Resolve<ReadArgs> for GetSecret {
  async fn resolve(
    self,
    ReadArgs { client }: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    client.admin_only()?;
    let secret = query::secret::get_secret(&self.id.0).await?;
    decrypt_secret(secret).await
  }
}

impl Resolve<ReadArgs> for FindSecret {
  async fn resolve(
    self,
    ReadArgs { client }: &ReadArgs,
  ) -> Result<Self::Response, Self::Error> {
    client.admin_only()?;
    let secret = query::secret::find_secret(self).await?;
    decrypt_secret(secret).await
  }
}
