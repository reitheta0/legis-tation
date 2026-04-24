/*fn capitalize_lone_word(string: &mut String) {
  if let Some(first_letter) = string.as_bytes().first_mut() {
    if first_letter.is_ascii_lowercase() {
      *first_letter = first_letter.to_ascii_uppercase();
    }
  }
}*/
fn capitalize_lone_word(s: &str) -> String {
  let mut chars = s.chars();
  match chars.next() {
    None => String::new(),
    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
  }
}
pub(crate) fn handle_text(string: &mut String) {
  string
    .split('_')
    .flat_map(|string| capitalize_lone_word(string));
}
