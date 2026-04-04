use mogh_auth_client::openapi::MoghAuthApi;
use utoipa::OpenApi;

mod read {
  pub use crate::api::read::{
    device::*, encryption_key::*, filesystem::*, node::*,
    onboarding_key::*, secret::*, *,
  };
}

mod write {
  pub use crate::api::write::{
    device::*, encryption_key::*, filesystem::*, node::*,
    onboarding_key::*, secret::*,
  };
}

#[derive(OpenApi)]
#[openapi(
  nest(
    (path = "/auth", api = MoghAuthApi)
  ),
  paths(
    // ======
    //  READ
    // ======
    read::get_version,
    read::get_username,
    // DEVICE
    read::list_devices,
    read::get_device,
    // ONBOARDING KEY
    read::list_onboarding_keys,
    read::get_onboarding_key,
    // FILESYSTEM
    read::list_filesystems,
    // NODE
    read::list_nodes,
    read::get_node,
    read::find_node,
    // SECRET
    read::list_secrets,
    read::get_secret,
    read::find_secret,
    // ENCRYPTION KEY
    read::list_encryption_keys,
    read::get_encryption_key,
    // =======
    //  WRITE
    // =======
    // DEVICE
    write::create_device,
    write::update_device,
    write::delete_device,
    write::batch_delete_devices,
    // ONBOARDING KEY
    write::create_onboarding_key,
    write::update_onboarding_key,
    write::delete_onboarding_key,
    write::batch_delete_onboarding_keys,
    // FILESYSTEM
    write::create_filesystem,
    write::update_filesystem,
    write::delete_filesystem,
    // NODE
    write::create_node,
    write::update_node,
    write::update_node_data,
    write::update_node_encryption_key,
    write::rotate_node_envelope_key,
    write::delete_node,
    write::batch_delete_nodes,
    // SECRET
    write::create_secret,
    write::update_secret,
    write::update_secret_data,
    write::update_secret_encryption_key,
    write::rotate_secret_envelope_key,
    write::delete_secret,
    write::batch_delete_secrets,
    // ENCRYPTION KEY
    write::create_encryption_key,
    write::update_encryption_key,
    write::initialize_encryption_key,
  ),
)]
pub struct CicadaApi;
