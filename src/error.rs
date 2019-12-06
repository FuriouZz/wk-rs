use std::{
  io,
  fmt,
  str::Utf8Error,
};

#[derive(Debug)]
pub enum Error {
  Other(Box<dyn std::error::Error>),
  ImportError(String),
  CommandError(String),
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Error::ImportError(s) => write!(f, "[Import] {}", s),
      Error::CommandError(s) => write!(f, "[Command] {}", s),
      Error::Other(e) => write!(f, "[Other] {}", e),
    }
  }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
  fn from(value: std::io::Error) -> Self {
    Error::Other(Box::new(value))
  }
}

impl From<serde_yaml::Error> for Error {
  fn from(value: serde_yaml::Error) -> Self {
    Error::Other(Box::new(value))
  }
}

impl From<Utf8Error> for Error {
  fn from(value: Utf8Error) -> Self {
    Error::Other(Box::new(value))
  }
}
