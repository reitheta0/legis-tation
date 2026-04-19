use std::{fs, path::Path};

use kdl::{KdlDocument, KdlError};

pub fn read_kdl(filename: &Path) -> Result<KdlDocument, KdlError> {
  match fs::read_to_string(filename) {
    Ok(strng) => match strng.parse() {
      Ok(doc) => Ok(doc),
      Err(error) => Err(ProgressError::ParseError),
    },

    Err(error) => Err(ProgressError::ReadError),
  }
  /*.parse() {
    Ok(doc) => Ok(doc),
    Err(error) => Err(error),
  }*/
}
