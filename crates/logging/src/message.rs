use crate::LoggingStatus;

///
/// These are structs that gives a message to report to the files.
///
/// # Parts:
///
///
///   Logging Status: Gives the status to append to the log file.
///
///
///   Message: The actual text that is used for logging the file.
///
///
#[derives::transform]
pub struct StatusMessage {
  #[default(LoggingStatus::default())]
  logging_status: LoggingStatus,
  #[default(String::new())]
  message: String,
}
