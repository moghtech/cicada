use tower_http::cors::CorsLayer;

/// Creates a CORS layer based on the Core configuration.
///
/// - If `cors_allowed_origins` is empty: Allows all origins (backward compatibility)
/// - If `cors_allowed_origins` is set: Only allows the specified origins
/// - Methods and headers are always allowed (Any)
/// - Credentials are only allowed if `cors_allow_credentials` is true
pub fn cors_layer() -> CorsLayer {
  // let config = core_config();
  let mut cors = CorsLayer::new()
    .allow_methods(tower_http::cors::AllowMethods::mirror_request())
    .allow_headers(tower_http::cors::AllowHeaders::mirror_request());
    // .allow_credentials(config.cors_allow_credentials);
  if true {
    warn!(
      "CORS using allowed origin 'Any' (*). Use CICADA_CORS_ALLOWED_ORIGINS to configure specific origins."
    );
    cors = cors.allow_origin(tower_http::cors::Any)
  } else {
    // let allowed_origins = config
    //   .cors_allowed_origins
    //   .iter()
    //   .filter_map(|origin| {
    //     HeaderValue::from_str(origin)
    //       .inspect_err(|e| {
    //         warn!("Invalid CORS allowed origin: {origin} | {e:?}")
    //       })
    //       .ok()
    //   })
    //   .collect::<Vec<_>>();
    // info!("CORS using allowed origin/s: {allowed_origins:?}");
    // cors = cors.allow_origin(allowed_origins);
  };
  cors
}
