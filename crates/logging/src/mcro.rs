use crate::LoggingStatus;

use crate::StatusMessage;

use crate::log_file;

pub fn quick_log(message: &str, char_status: char, is_json: bool) {
  let status: LoggingStatus;
  match char_status {
    'd' => status = LoggingStatus::Debug,
    'i' => status = LoggingStatus::Info,
    'w' => status = LoggingStatus::Warn,
    'e' => status = LoggingStatus::Error,
    _ => status = LoggingStatus::Trace,
  }
  log_file(
    is_json,
    StatusMessage::new(LoggingStatus::Info, message.to_string()),
  );
}
