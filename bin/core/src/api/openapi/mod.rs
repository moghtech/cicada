use mogh_auth_server::api::openapi::MoghAuthApi;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable as _};

mod read {
  pub use crate::api::read::{device::*, filesystem::*, node::*, *};
}

mod write {
  pub use crate::api::write::{device::*, filesystem::*, node::*};
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
    // DEVICE
    read::list_devices,
    // FILESYSTEM
    read::list_filesystems,
    // NODE
    read::list_nodes,
    read::get_node,
    read::find_node,
    // =======
    //  WRITE
    // =======
    // DEVICE
    write::create_device,
    write::update_device,
    write::delete_device,
    // FILESYSTEM
    write::create_filesystem,
    write::update_filesystem,
    write::delete_filesystem,
    // NODE
    write::create_node,
    write::update_node,
    write::delete_node,
  ),
)]
struct CicadaApi;

pub fn serve_docs() -> Scalar<utoipa::openapi::OpenApi> {
  Scalar::with_url("/docs", CicadaApi::openapi())
    .custom_html(include_str!("docs.html"))
}
