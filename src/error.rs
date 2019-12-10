use std::{fmt, io, str::Utf8Error};

#[derive(Debug)]
pub enum Error {
  Std(Box<dyn std::error::Error>),
  Import(String),
  Command(String),
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Error::Import(s) => write!(f, "[Import] {}", s),
      Error::Command(s) => write!(f, "[Command] {}", s),
      Error::Std(e) => write!(f, "[Std] {}", e),
    }
  }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
  fn from(value: std::io::Error) -> Self {
    Error::Std(Box::new(value))
  }
}

impl From<serde_yaml::Error> for Error {
  fn from(value: serde_yaml::Error) -> Self {
    Error::Std(Box::new(value))
  }
}

impl From<Utf8Error> for Error {
  fn from(value: Utf8Error) -> Self {
    Error::Std(Box::new(value))
  }
}
