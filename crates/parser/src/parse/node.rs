use crate::parse::entry::NodeEntryState;
use derives::transform_s;

//TODO: add use crate::data::markup::entry::NodeEntryState;

#[transform_s]
pub struct NodeState {
  text: Option<String>,
  children: Option<Box<Vec<NodeState>>>,
  entries: Option<Vec<NodeEntryState>>,
}

impl NodeState {
  pub fn initial_node_depth(&self, depth: u32) -> u32 {
    match self.get_children() {
      Some(children) => children
        .iter()
        .map(|child| child.initial_node_depth(depth + 1))
        .max()
        .unwrap(),
      None => depth,
    }
  }

  pub fn get_depth(&self) -> u32 {
    self.initial_node_depth(0)
  }
}
