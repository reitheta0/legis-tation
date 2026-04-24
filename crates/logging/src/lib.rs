///
///
/// Logger Modules :
///
///
/// This checks the logs.
///
pub mod check;
pub mod grab;
///
/// This gives performance statistics
///

///
/// This actually does the function to log to the log files.
///
pub mod log;
pub mod macros;
///
/// This gives the macro to log the files with the character codes.
///
pub mod mcro;
///
/// This checks the logs
///
pub mod message;
pub mod setup;
///
/// This gives a status struct to be able to append this to the file.
///
pub mod status;

//pub use check::check_logs;
pub use log::log_file;
pub use mcro::quick_log;
pub use message::StatusMessage;
pub use status::LoggingStatus;
