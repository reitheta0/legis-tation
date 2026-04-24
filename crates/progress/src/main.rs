use std::path::Path;

use logging::quick_log;

use node::nodes::process::grab_parsed_kdl_document;

pub fn main() {
  //  path("/Users/aylabennett/Programming/Rust Projects/legis-tation/crates/progress/input.kdl");
  /*match read_and_write(
    Path::new(
      "/Users/aylabennett/Programming/Rust Projects/legis-tation/crates/progress/input.kdl",
    ),
    Path::new(""),
  ) {
    Ok(_) => {
      println!();
      quick_log("Ran Successfully!", 't', false);
    }
    Err(error) => {
      eprintln!("{}", error);
      dbg!(&error);
      quick_log(&format!("Ran into an error:{error}!"), 'e', false);
    }
  }*/
}
