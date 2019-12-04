#[derive(Debug)]
pub enum Error {
  IoError(std::io::Error),
  YamlError(serde_yaml::Error),
  StrError(std::str::Utf8Error),
  StringEmpty,
  ExtendMissingCommand(String),
  ExtendConcurrent(String),
  LookupError(String),
  CommandError(String),
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::StrError(_) => write!(f, "Cannot create a command from empty string"),
      Error::YamlError(_) => write!(f, "Cannot parse yaml"),
      Error::IoError(_) => write!(f, "Cannot read yaml file"),
      Error::StringEmpty => write!(f, "Cannot convert string to command. String is empty."),
      Error::ExtendMissingCommand(s) => write!(f, "Cannot extend '{}', it is missing.", s),
      Error::ExtendConcurrent(s) => write!(f, "Cannot extend '{}', it is concurrent.", s),
      Error::LookupError(s) => write!(f, "Lookup failed: {}", s),
      Error::CommandError(s) => write!(f, "Command failed: {}", s),
    }
  }
}

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      Error::StrError(ref e) => Some(e),
      Error::YamlError(ref e) => Some(e),
      Error::IoError(ref e) => Some(e),
      _ => None,
    }
  }
}

impl From<std::io::Error> for Error {
  fn from(value: std::io::Error) -> Self {
    Error::IoError(value)
  }
}

impl From<serde_yaml::Error> for Error {
  fn from(value: serde_yaml::Error) -> Self {
    Error::YamlError(value)
  }
}

impl From<std::str::Utf8Error> for Error {
  fn from(value: std::str::Utf8Error) -> Self {
    Error::StrError(value)
  }
}
