use tracing::instrument;
use tracing_appender::rolling;
use tracing_subscriber::{EnvFilter, fmt};

use crate::logger::{message::StatusMessage, status::LoggingStatus};

///
///
/// Sets up logging for jsons to see errors and get accurate debbuging.
///
///
/// These give the actual functions that make it tick.
///
///
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

impl StatusMessage {
  pub fn add_to_file(&self) {
    match self.get_logging_status() {
      LoggingStatus::Trace => {
        tracing::trace!("{}", self.get_message())
      }
      LoggingStatus::Debug => {
        tracing::debug!("{}", self.get_message())
      }
      LoggingStatus::Info => {
        tracing::info!("{}", self.get_message())
      }
      LoggingStatus::Warn => {
        tracing::warn!("{}", self.get_message())
      }
      LoggingStatus::Error => {
        tracing::error!("{}", self.get_message())
      }
    }
  }
}
