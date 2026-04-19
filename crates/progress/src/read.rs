use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::tree::ProgressTreeState;

pub fn read_to_tree() -> ProgressTreeState {
  let file = File::open("file.txt").expect("Failed to open file");
  let reader = BufReader::new(file);
  let mut progress_tree_state = ProgressTreeState::new(Vec::new());
  for line in reader.lines() {
    unwrapped_line = line.unwrap().trim();
    l
    progress_tree_state.push_node(line.unwrap());
  }
  progress_tree_state
}
