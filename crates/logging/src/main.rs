use logging::{LoggingStatus, StatusMessage, grab, log_file};

pub fn main() {
  let my_message = StatusMessage::new(LoggingStatus::Info, "Hello World".to_string());

  log_file(
    true,
    StatusMessage::new(LoggingStatus::Info, "Hello World".to_string()),
  );
  log_file(
    true,
    StatusMessage::new(LoggingStatus::Trace, "Hello World".to_string()),
  );
  log_file(
    true,
    StatusMessage::new(LoggingStatus::Error, "Hello World".to_string()),
  );
}
