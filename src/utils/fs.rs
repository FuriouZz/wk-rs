use std::io::Read;

pub struct Reader;

impl Reader {
  pub fn text<P>(path: P) -> Result<String, std::io::Error>
  where
    P: AsRef<std::path::Path>,
  {
    let mut file = std::fs::File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
  }
}
