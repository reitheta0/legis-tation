use crate::{leaf::LineLeafState, node::NodeState, templates::depth_to_text};

use derives::transform_s;

#[transform_s]
pub struct ProgressNodeState {
  node: NodeState,
  line_state: LineLeafState,
}

impl ProgressNodeState {
  pub fn to_string(&self) -> String {
    let state = self.get_line_state();
    let depth = self.get_depth();
    let suffix: String;
    match state {
      LineLeafState::Leaf(state) => {
        let inserted_char: char = [' ', 'X'][*state.get_completed() as usize];
        let text = state.get_text();
        suffix = format!("||= {text} --> [{inserted_char}] ");
      }
      LineLeafState::Changing => {
        suffix = "|| >=> []".to_string();
      }
      LineLeafState::End => {
        suffix = "||     []".to_string();
      }
      LineLeafState::Monostate => {
        suffix = "||".to_string();
      }
    }
    if depth != 0 {
      return format!("  {}", "||     ".repeat((depth - 1) as usize));
    } else {
      return format!("  {}", suffix);
    }
  }
  pub fn to_text(&self) -> [String; 2] {
    [
      self.to_string(),
      ProgressNodeState::new(*self.get_node(), LineLeafState::Monostate).to_string(),
    ]
  }
}
