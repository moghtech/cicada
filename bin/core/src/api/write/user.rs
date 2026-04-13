use cicada_client::api::write::{
  BatchDeleteUsers, CreateUser, DeleteUser, UpdateUser,
};
use mogh_auth_server::AuthImpl;
use mogh_error::{AddStatusCodeError, StatusCode, anyhow::anyhow};
use mogh_resolver::Resolve;

use crate::{api::write::WriteArgs, auth::CicadaAuthImpl, db::query};

impl Resolve<WriteArgs> for CreateUser {
  async fn resolve(
    mut self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    client.admin_only()?;

    let password = self.password.unwrap_or_default();

    CicadaAuthImpl.validate_username(&self.username)?;
    CicadaAuthImpl.validate_password(&password)?;

    self.password = bcrypt::hash(
      password.as_bytes(),
      CicadaAuthImpl.local_auth_bcrypt_cost(),
    )?
    .into();

    query::user::create_user(self).await
  }
}

//

impl Resolve<WriteArgs> for UpdateUser {
  async fn resolve(
    mut self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    client.admin_only()?;
    // Make sure user can't un-enable or un-admin themselves
    if self.id == client.as_user()?.id {
      self.enabled = None;
      self.admin = None;
      self.super_admin = None;
    }
    query::user::update_user(self).await
  }
}

//

impl Resolve<WriteArgs> for DeleteUser {
  async fn resolve(
    self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    client.admin_only()?;
    if client.as_user()?.id == self.id {
      return Err(
        anyhow!("User not allowed to delete self")
          .status_code(StatusCode::FORBIDDEN),
      );
    }
    query::user::delete_user(self.id).await
  }
}

//

impl Resolve<WriteArgs> for BatchDeleteUsers {
  async fn resolve(
    self,
    WriteArgs { client }: &WriteArgs,
  ) -> Result<Self::Response, Self::Error> {
    client.admin_only()?;
    let user = client.as_user()?;
    query::user::batch_delete_users(
      // Ensure user doesn't delete self
      self.ids.into_iter().filter(|id| *id != user.id).collect(),
    )
    .await
  }
}
