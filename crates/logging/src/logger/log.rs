use tracing::instrument;
use tracing_appender::rolling;
use tracing_subscriber::{EnvFilter, fmt};

/// Sets up logging for jsons to see errors and get accurate debbuging.
#[instrument]
pub fn setup_log_file(log_json: bool) {
  let log_file = rolling::daily("./././log", "info");

  let layer = fmt()
    .with_writer(log_file)
    .with_level(true)
    .with_target(false)
    .with_max_level(tracing::Level::TRACE)
    .with_ansi(false)
    .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("backend=info,tower_https=info,backend::handlers::http::telemetry_handler=info,tower_http=info"))
  );

  if log_json {
    layer.json().init()
  } else {
    layer.compact().init();
  }
}
