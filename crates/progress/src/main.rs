use std::path::Path;

use logging::quick_log;

use progress::write::read_and_write;

pub fn main() {
  match read_and_write(Path::new(""), Path::new("")) {
    Ok(_) => {
      println!();
      quick_log("Ran Successfully!", 't', false);
    }
    Err(error) => {
      eprintln!("{}", error);
      quick_log("Ran into an error!", 'e', false);
    }
  }
}
