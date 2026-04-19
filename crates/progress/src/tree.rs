use std::fs::File;

use thiserror::Error;

use std::io::Write;

use derives::transform_s;

use derive_more::IntoIterator;

use derive_new::new;

use crate::{error::ProgressError, progression::ProgressNodeState};

#[derive(IntoIterator)]
#[transform_s]
pub struct ProgressTreeState {
  root_nodes: Vec<(ProgressNodeState, u32)>,
}

#[derive(Clone)]
pub struct ProgressTreeStateResult(pub Result<ProgressTreeState, ProgressError>);

impl From<Result<Vec<(ProgressNodeState, u32)>, ProgressError>> for ProgressTreeStateResult {
  fn from(vec_result: Result<Vec<(ProgressNodeState, u32)>, ProgressError>) -> Self {
    let state = vec_result.map(|root_nodes| ProgressTreeState { root_nodes });
    ProgressTreeStateResult(state)
  }
}

///
///
/// This is the function that actually writes the gathered notes to the file.
///
///
impl ProgressTreeState {
  pub fn write_nodes(self, file: &mut File) {
    writeln!(file, "  []");
    for (node, depth) in self.into_iter() {
      let text = node.to_text(depth);
      writeln!(file, "{}", text[0]);
      writeln!(file, "{}", text[1]);
    }
    writeln!(file, "  []");
  }
}
