use node::nodes::node::NodeState;

use kdl::KdlError;

use std::path::Path;

use node::nodes::process::grab_parsed_kdl_document;

use crate::error::ProgressError;
use crate::leaf::LineLeafState;

use crate::progression::ProgressNodeState;

use crate::tree::{ProgressTreeState, ProgressTreeStateResult};

fn grab_state_from_vector(
  this_object: &NodeState,
  next_object: &NodeState,
  progress_tree: &mut ProgressTreeState,
) -> ProgressTreeState {
  let depth_difference: u32;
  if this_object.get_depth() > next_object.get_depth() {
    depth_difference = this_object.get_depth() - next_object.get_depth();
    for index in 0..depth_difference {
      let mut root_nodes = progress_tree.get_root_nodes().clone();
      root_nodes.push((
        ProgressNodeState::new(this_object.clone(), LineLeafState::End),
        3,
      ));
    }
    progress_tree.clone()
  } else if this_object.get_depth() == next_object.get_depth() {
    depth_difference = 0;
    progress_tree.clone()
  } else {
    depth_difference = 1;
    let mut root_nodes = progress_tree.get_root_nodes().clone();
    root_nodes.push((
      ProgressNodeState::new(this_object.clone(), LineLeafState::Changing),
      this_object.get_depth(),
    ));
    progress_tree.set_root_nodes(root_nodes);
    progress_tree.clone()
  }
}

pub fn read_to_tree(input_filename: &Path) -> Result<ProgressTreeState, ProgressError> {
  let vector: Result<Vec<NodeState>, KdlError> = grab_parsed_kdl_document(input_filename);
  let mut progress_tree = ProgressTreeState::new(Vec::new());
  match vector {
    Ok(vect) => {
      let len = vect.len();
      for (index, object) in vect[0..len - 1].iter().enumerate() {
        progress_tree = grab_state_from_vector(&object, &vect[index + 1], &mut progress_tree);
      }
      Ok(progress_tree)
    }
    Err(error) => Err(ProgressError::KdlError),
  }
}
