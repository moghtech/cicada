use axum::{Router, routing::get};
use tower_http::services::{ServeDir, ServeFile};

use crate::config::core_config;

mod openapi;
mod read;
mod response;
mod write;

#[derive(serde::Deserialize)]
struct Variant {
  variant: String,
}

pub fn app() -> Router {
  let config = core_config();

  // Setup static ui services
  let ui_path = &config.ui_path;
  let ui_index = ServeFile::new(format!("{ui_path}/index.html"));
  let serve_ui =
    ServeDir::new(ui_path).not_found_service(ui_index.clone());

  Router::new()
    .merge(openapi::serve_docs())
    .route("/version", get(|| async { env!("CARGO_PKG_VERSION") }))
    // .nest("/auth", auth::router())
    // .nest("/user", user::router())
    .nest("/read", read::router())
    .nest("/write", write::router())
    // .nest("/listener", listener::router())
    // .nest("/client", ts_client::router())
    .fallback_service(serve_ui)
    .layer(mogh_server::session::layer(config))
    .layer(mogh_server::cors::layer(config))
}
