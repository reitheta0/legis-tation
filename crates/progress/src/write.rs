use std::fs::File;

use std::path::Path;

use node::nodes::error::ProgressError;

use crate::read::read_to_tree;

pub fn read_and_write(input_filename: &Path, output_filename: &Path) -> Result<(), ProgressError> {
  let read_data = read_to_tree(input_filename);
  let write_file = File::open(output_filename);
  match write_file {
    Ok(mut writing_file) => {
      //read_data.write_nodes(&mut writing_file);
      /*match read_data {
        Ok(vect) => Ok(vect.write_nodes(&mut writing_file)),
        Err(error) => Err(ProgressError::KdlError),
      }*/
      read_data?.write_nodes(&mut writing_file);
      Ok(())
    }
    Err(_) => Err(ProgressError::ReadError),
  }
}
