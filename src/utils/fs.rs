use std::io::Read;
use crate::error::Error;

pub struct Reader;

impl Reader {

  pub fn text<P>(path: P) -> Result<String, std::io::Error>
  where P: AsRef<std::path::Path>
  {
    let mut file = std::fs::File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
  }

  #[allow(dead_code)]
  pub fn toml_value<P>(path: P) -> Result<toml::Value, Box<dyn std::error::Error>>
  where P: AsRef<std::path::Path>
  {
    let result = Reader::text(path)?;
    let data = result.as_str().parse::<toml::Value>()?;
    Ok(data)
  }

}

#[allow(dead_code)]
pub fn fetch<P>(dir_path: P, pattern: &str) -> Result<(), std::io::Error>
where P: AsRef<std::path::Path> {

  let d = dir_path.as_ref();
  let d = d.join(pattern);

  let dir = std::fs::read_dir(dir_path)?;

  let mut it = dir.into_iter();
  while let Some(item) = it.next() {
    if let Ok(entry) = item {
      let entry_path = entry.path();
      let entry_str = entry_path.to_str().unwrap();
      let b = d.to_str().unwrap() == entry_str;
      println!("{} {} {}", entry_str, entry_str.len(), b);
      if entry_path.is_file() && b {
        return Ok(());
      }
    }
  }

  Ok(())
}