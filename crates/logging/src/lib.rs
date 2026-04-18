///
///
/// Logger Modules :
///
///
/// This checks the logs.
///
pub mod check;
///
/// This gives performance statistics
///
pub mod flame;

///
/// This actually does the function to log to the log files.
///
pub mod log;
///
/// This checks the logs
///
pub mod message;
///
/// This gives a status struct to be able to append this to the file.
///
pub mod status;

pub use check::check_logs;
pub use flame::setup_inferno_file;
pub use log::setup_log_file;
pub use message::StatusMessage;
pub use status::LoggingStatus;
