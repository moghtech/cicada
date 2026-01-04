use axum::{Router, routing::get};
use mogh_server::{
  cors::cors_layer, session::memory_session_layer,
  ui::serve_static_ui,
};

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
  Router::new()
    .merge(openapi::serve_docs())
    .route("/version", get(|| async { env!("CARGO_PKG_VERSION") }))
    // .nest("/auth", auth::router())
    // .nest("/user", user::router())
    .nest("/read", read::router())
    .nest("/write", write::router())
    // .nest("/listener", listener::router())
    // .nest("/client", ts_client::router())
    .fallback_service(serve_static_ui(&config.ui_path))
    .layer(cors_layer(config))
    .layer(memory_session_layer(config))
}
