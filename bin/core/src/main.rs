#[macro_use]
extern crate tracing;

use std::{net::SocketAddr, str::FromStr as _};

use anyhow::Context as _;
use tracing::Instrument as _;

mod api;
mod config;
mod db;

async fn app() -> anyhow::Result<()> {
  dotenvy::dotenv().ok();
  tracing_subscriber::fmt::init();

  let startup_span = info_span!("CoreStartup");

  async {
    info!("Cicada Core version: v{}", env!("CARGO_PKG_VERSION"));

    rustls::crypto::aws_lc_rs::default_provider()
      .install_default()
      .map_err(|_| {
        anyhow::Error::msg("Failed to install tls crypto provider")
      })?;

    db::init().await?;

    // match (
    //   config.pretty_startup_config,
    //   config.unsafe_unsanitized_startup_config,
    // ) {
    //   (true, true) => info!("{:#?}", config),
    //   (true, false) => info!("{:#?}", config.sanitized()),
    //   (false, true) => info!("{:?}", config),
    //   (false, false) => info!("{:?}", config.sanitized()),
    // }

    // Init + log public key. Will crash if invalid private key here.
    // info!("Public Key: {}", core_keys().load().public);

    anyhow::Ok(())
  }
  .instrument(startup_span)
  .await?;

  let app =
    api::app().into_make_service_with_connect_info::<SocketAddr>();

  //   let addr =
  //     format!("{}:{}", core_config().bind_ip, core_config().port);
  let socket_addr = SocketAddr::from_str("[::]:9220")
    .context("failed to parse listen address")?;

  //   let handle = Handle::new();
  //   tokio::spawn({
  //     // We can use a handle for the server, and wait until
  //     // the handle is listening before running actions
  //     let handle = handle.clone();
  //     async move {
  //       handle.listening().await;
  //       startup::run_startup_actions().await;
  //     }
  //   });

  if false {
    // info!("🔒 Core SSL Enabled");
    // info!("Cicada Core starting on https://{socket_addr}");
    // let ssl_config = RustlsConfig::from_pem_file(
    //   &config.ssl_cert_file,
    //   &config.ssl_key_file,
    // )
    // .await
    // .context("Invalid ssl cert / key")?;
    // axum_server::bind_rustls(socket_addr, ssl_config)
    //   .handle(handle)
    //   .serve(app)
    //   .await
    //   .context("failed to start https server")
    todo!()
  } else {
    info!("🔓 Core SSL Disabled");
    info!("Cicada Core starting on http://{socket_addr}");
    axum_server::bind(socket_addr)
      //   .handle(handle)
      .serve(app)
      .await
      .context("failed to start http server")
  }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let mut term_signal = tokio::signal::unix::signal(
    tokio::signal::unix::SignalKind::terminate(),
  )?;
  tokio::select! {
    res = tokio::spawn(app()) => res?,
    _ = term_signal.recv() => Ok(()),
  }
}
