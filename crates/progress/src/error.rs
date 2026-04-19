use derive_more::Display;

use derives::transform_d;

use thiserror::Error;

#[derive(Error, Display)]
#[transform_d]
pub enum ProgressError {
  #[default]
  ReadError,
  WriteError,
  KdlError,
  ParseError,
}
