use std::fs;

use kdl::KdlDocument;

pub fn read_kdl(filename: &str) -> KdlDocument {
  let text = fs::read_to_string(filename).unwrap();
  text.parse().unwrap()
}
