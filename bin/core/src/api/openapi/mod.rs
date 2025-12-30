use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable as _};

use super::{read, write};

#[derive(OpenApi)]
#[openapi(paths(
  // ======
  //  READ
  // ======
  read::get_version,
  // FILESYSTEM
  read::filesystem::list_filesystems,
  // NODE
  read::node::list_nodes,
  read::node::get_node,
  read::node::find_node,
  // =======
  //  WRITE
  // =======
  // FILESYSTEM
  write::filesystem::create_filesystem,
  write::filesystem::update_filesystem,
  // NODE
  write::node::create_node,
  write::node::update_node,
))]
struct CicadaApi;

pub fn serve_docs() -> Scalar<utoipa::openapi::OpenApi> {
  Scalar::with_url("/openapi", CicadaApi::openapi())
    .custom_html(include_str!("scalar.html"))
}
