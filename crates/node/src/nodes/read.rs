use std::{fs, path::Path};

use kdl::{KdlDocument, KdlError};

use crate::nodes::error::ProgressError;

pub fn read_kdl(filename: &Path) -> Result<KdlDocument, ProgressError> {
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
