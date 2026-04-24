use std::path::Path;

use logging::quick_log;

use node::nodes::{child::grab_children, error::ProgressError, process::grab_parsed_kdl_document};

fn path_incomplete(path: &str) -> Result<(), ProgressError> {
  match grab_parsed_kdl_document(Path::new(path)) {
    Ok(vector) => {
      quick_log("WHAT THE ACTUAL FUCK?!?!?!?!?!?", 'i', false);
    }
    Err(error) => {
      eprintln!("{}", error);
      dbg!(&error);
      println!("Ran into an error:{error}!");
      quick_log(&format!("Ran into an error:{error}!"), 'e', false);
      quick_log("WHAT THE ACTUAL SHIT?!?!?!?!?!?", 'i', false);
      return Err(error);
    }
  }
  if let Ok(vector) = grab_parsed_kdl_document(Path::new(
    "/Users/aylabennett/Programming/Rust Projects/legis-tation/crates/progress/input.kdl",
  )) {
    quick_log("WHAT THE ACTUAL GOONING?!?!?!?!?!?", 't', false);
    quick_log(
      &format!(
        "Here is the number of children of the kdl document: {}!",
        grab_children(vector.clone()).len()
      ),
      't',
      false,
    );
    for node in grab_children(vector) {
      println!("Ran Successfully: Here's the node: {node:?}!");
      if let Some(text) = node.get_text() {
        println!("Ran Successfully: Here's the text: {text}!");
        quick_log(
          &format!("Ran Successfully: Here's the text: {text}!"),
          't',
          false,
        );
      } else {
        println!("Ran Successfully: There's no text for this one.");
        quick_log(
          "Ran Successfully: There's no text for this one.",
          'i',
          false,
        );
      }
    }
  }
  return Ok(());
}

#[test]
fn path() -> Result<(), ProgressError> {
  quick_log("WHAT THE ACTUAL GOONING?!?!?!?!?!?", 't', false);
  path_incomplete(
    "/Users/aylabennett/Programming/Rust Projects/legis-tation/crates/progress/input.kdl",
  )
}
