use derives::transform_s;

#[transform_s]
pub struct NodeEntryState {
  name: Option<String>,
  value: Option<String>,
}
