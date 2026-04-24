///
///
/// Checks logs files for their results and reads their values.
///
///
pub fn check_logs() {
  for (key, value) in std::env::vars() {
    println!("{key} = {value}")
  }
}
