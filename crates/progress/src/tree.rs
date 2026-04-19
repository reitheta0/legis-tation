use std::fs::File;

use derives::transform_s;

use derive_more::IntoIterator;

use crate::{node::NodeState, progression::ProgressNodeState};

#[derive(IntoIterator)]
#[transform_s]
pub struct ProgressTreeState {
  root_nodes: Vec<ProgressNodeState>,
}

///
///
/// This is the function that actually writes the gathered notes to the file.
///
///
impl ProgressTreeState {
  pub fn write_nodes(self, file: File) {
    for node in self.to_iter() {
      let text = node.to_text();
      writeln!(file, text[0]);
      writeln!(file, text[1]);
    }
  }

  pub fn push_node(self, node: ProgressNodeState) {
    self.get_root_nodes().push(ProgressNodeState)
  }
}
