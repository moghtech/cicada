use cicada_client::openapi::CicadaApi;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable as _};

pub fn serve_docs() -> Scalar<utoipa::openapi::OpenApi> {
  Scalar::with_url("/docs", CicadaApi::openapi())
    .custom_html(include_str!("docs.html"))
}
