use kdl::{KdlDocument, KdlValue};
use node::nodes::read::read_kdl;

use crate::parse::{entry::NodeEntryState, node::NodeState};

//TODO: use crate::data::markup::{entry::NodeEntryState, essay::read_kdl, node::NodeState};

pub fn parse_kdl_document(kdl_value: KdlDocument) -> Vec<NodeState> {
  let mut vector: Vec<NodeState> = Vec::new();

  for node in kdl_value.nodes() {
    let name = node.name().value().to_string();
    let entries: Vec<NodeEntryState> = node
      .entries()
      .iter()
      .map(|entry| match entry.name() {
        Some(name) => NodeEntryState::new(Some(name.to_string()), Some(entry.value().to_string())),
        None => NodeEntryState::new(None, Some(entry.value().to_string())),
      })
      .collect();
    let bool_entries = entries.clone();
    let option_entries = if bool_entries.is_empty() {
      None
    } else {
      Some(entries)
    };
    match &node.children() {
      Some(document) => vector.push(NodeState::new(
        Some(name),
        Some(Box::new(parse_kdl_document(document.clone().clone()))),
        option_entries,
      )),
      None => {}
    }
  }
  vector
}

pub fn grab_parsed_kdl_document(filename: &str) -> Vec<NodeState> {
  parse_kdl_document(read_kdl(filename))
}
