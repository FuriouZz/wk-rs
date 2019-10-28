use std::io::Read;

#[derive(Debug)]
pub enum FileError {
  InvalidData
}

impl From<std::io::Error> for FileError {
  fn from(_: std::io::Error) -> FileError
  {
    FileError::InvalidData
  }
}

impl From<toml::de::Error> for FileError {
  fn from(_: toml::de::Error) -> FileError
  {
    FileError::InvalidData
  }
}

pub struct Reader;

impl Reader {

  pub fn text<P>(path: P) -> Result<String, FileError>
  where P: AsRef<std::path::Path>
  {
    let mut file = std::fs::File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
  }

  pub fn toml_value<P>(path: P) -> Result<toml::Value, FileError>
  where P: AsRef<std::path::Path>
  {
    let result = Reader::text(path)?;
    let data = result.as_str().parse::<toml::Value>()?;
    Ok(data)
  }

}