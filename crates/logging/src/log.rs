use serde::ser::StdError;

use tracing::instrument;

use tracing_appender::rolling;

use tracing_subscriber::{EnvFilter, fmt};

use crate::{LoggingStatus, StatusMessage};

///
///
/// Sets up logging for jsons to see errors and get accurate debbuging.
///
///
/// These give the actual functions that make it tick.
///
///
//#[instrument]
pub fn log_file(
  log_json: bool,
  message: StatusMessage,
) -> Result<(), Box<dyn StdError + Send + Sync + 'static>> {
  let log_file = rolling::daily("./log", "info");
  let filter = EnvFilter::new("trace");
  let layer = fmt()
    .with_env_filter(filter)
    .with_writer(log_file)
    .with_level(true)
    .with_target(false)
    .with_ansi(false);

  let rtrn: Result<(), Box<dyn StdError + Send + Sync + 'static>>;

  if log_json {
    rtrn = layer.json().try_init();
  } else {
    rtrn = layer.compact().try_init();
  }

  match message.get_logging_status() {
    LoggingStatus::Trace => {
      tracing::trace!("{}", message.get_message())
    }
    LoggingStatus::Debug => {
      tracing::debug!("{}", message.get_message())
    }
    LoggingStatus::Info => {
      tracing::info!("{}", message.get_message())
    }
    LoggingStatus::Warn => {
      tracing::warn!("{}", message.get_message())
    }
    LoggingStatus::Error => {
      tracing::error!("{}", message.get_message())
    }
  }

  rtrn
}
