use axum::{
  Router,
  http::{HeaderName, HeaderValue},
  routing::get,
};
use tower_http::set_header::SetResponseHeaderLayer;
use tower_sessions::{
  Expiry, MemoryStore, SessionManagerLayer,
  cookie::{SameSite, time::Duration},
};
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

use crate::config::cors_layer;

mod read;
mod response;
mod write;

#[derive(serde::Deserialize)]
struct Variant {
  variant: String,
}

pub fn app() -> Router {
  // let config = core_config();

  // Setup static frontend services
  // let frontend_path = &config.frontend_path;
  // let frontend_index =
  //   ServeFile::new(format!("{frontend_path}/index.html"));
  // let serve_frontend = ServeDir::new(frontend_path)
  //   .not_found_service(frontend_index.clone());

  Router::new()
    .route("/version", get(|| async { env!("CARGO_PKG_VERSION") }))
    .merge(Scalar::with_url("/openapi", CicadaApi::openapi()))
    // .nest("/auth", auth::router())
    // .nest("/user", user::router())
    .nest("/read", read::router())
    .nest("/write", write::router())
    // .nest("/listener", listener::router())
    // .nest("/client", ts_client::router())
    .layer(memory_session_layer())
    // .fallback_service(serve_frontend)
    .layer(cors_layer())
    .layer(SetResponseHeaderLayer::overriding(
      HeaderName::from_static("x-content-type-options"),
      HeaderValue::from_static("nosniff"),
    ))
    .layer(SetResponseHeaderLayer::overriding(
      HeaderName::from_static("x-frame-options"),
      HeaderValue::from_static("DENY"),
    ))
    .layer(SetResponseHeaderLayer::overriding(
      HeaderName::from_static("x-xss-protection"),
      HeaderValue::from_static("1; mode=block"),
    ))
    .layer(SetResponseHeaderLayer::overriding(
      HeaderName::from_static("referrer-policy"),
      HeaderValue::from_static("strict-origin-when-cross-origin"),
    ))
}

const MEMORY_SESSION_EXPIRY_SECONDS: i64 = 60;

fn memory_session_layer() -> SessionManagerLayer<MemoryStore> {
  // let config = core_config();
  let layer = SessionManagerLayer::new(MemoryStore::default())
    .with_expiry(Expiry::OnInactivity(Duration::seconds(
      MEMORY_SESSION_EXPIRY_SECONDS,
    )))
    // .with_secure(config.host.starts_with("https://"))
    // Needs Lax in order for sessions to work
    // accross oauth redirects.
    .with_same_site(SameSite::Lax);
  // if let Some(domain) = core_host().and_then(|url| url.domain()) {
  //   layer = layer.with_domain(domain);
  // }
  layer
}

#[derive(OpenApi)]
#[openapi(paths(
  // ======
  //  READ
  // ======
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
