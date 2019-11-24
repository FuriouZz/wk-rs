use std::io::Read;

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
pub fn fetch<P>(dir_path: P, pattern: &str) -> Result<std::path::PathBuf, Box<dyn std::error::Error>>
where P: AsRef<std::path::Path> {
  let dir = std::fs::read_dir(dir_path)?;
  let re = regex::Regex::new(pattern).unwrap();

  let mut iter = dir
  .filter_map(|item: Result<std::fs::DirEntry, std::io::Error>| {
    match item {
      Ok(entry) => {
        let path = entry.path();
        if path.is_file() && re.is_match(path.to_str()?) {
          Some(path)
        } else {
          None
        }
      },
      _ => None
    }
  }).peekable();

  let p = iter.peek().unwrap();
  Ok(p.into())
}