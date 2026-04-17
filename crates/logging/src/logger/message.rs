use std::path::Path;

use crate::logger::status::LoggingStatus;

///
///
pub struct Log {
  logging_status: LoggingStatus,
  message: String,
  file: Path,
}
