use crate::nodes::node::NodeState;

pub fn grab_children(vector: Vec<NodeState>) -> Vec<NodeState> {
  vector
    .into_iter()
    .flat_map(|node| {
      let mut result = vec![node.clone()];
      if let Some(children) = node.get_children() {
        result.extend(grab_children(*children.clone()));
      }
      result
    })
    .collect::<Vec<NodeState>>()
}
