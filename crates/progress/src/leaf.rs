use derives::transform_s;

use crate::node::NodeState;

#[transform_s]
pub struct LeafState {
  text: String,
  completed: bool,
}

#[transform_s]
pub enum LineLeafState {
  Leaf(LeafState),
  Changing,
  End,
  #[default]
  Monostate,
}
