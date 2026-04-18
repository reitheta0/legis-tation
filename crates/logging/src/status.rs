///
/// Gives all the variants that can be used for logging.
///
/// # Types:
///
///
///   Trace: Provides small information about the application's operation that is normal, verbose
///
///   and harmless. It would be used to say things are going normally. If it were weather, it would
///   
///   be pleasant and sunny.
///
///
///   Debug: Debug messages that provide detailed information about the application's
///
///   internal state as well as tangible problems and is used for debugging and dev
///
///   builds only. If it were weather, it would be partially cloudy and misty.
///
///
///   Info: Informational messages that provide general information about the application's
///
///   operation and indicates something might be a little strange. If it were weather, it would be
///
///   Raining with light thunder.
///
///
///   Warn: Warning messages that indicate potential issues or unexpected behavior.
///
///   If it were weather, it would be a dangerous thunderstorm.
///
///
///   Error: Error messages that indicate a problem with the application's operation.
///
///   If it were weather, it would be a hurricane.
///
///
#[derives::transform]
pub enum LoggingStatus {
  #[default]
  Trace,
  Debug,
  Info,
  Warn,
  Error,
}
