use mogh_auth_server::api::openapi::MoghAuthApi;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable as _};

use super::{read as _read, write as _write};

mod read {
  pub use super::_read::{filesystem::*, node::*, *};
}

mod write {
  pub use super::_write::{filesystem::*, node::*};
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
    // FILESYSTEM
    read::list_filesystems,
    // NODE
    read::list_nodes,
    read::get_node,
    read::find_node,
    // =======
    //  WRITE
    // =======
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
