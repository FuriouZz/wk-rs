use crate::error::Error;
use std::{path::Path, path::PathBuf};

const FILES: [&'static str; 3] = ["commands.yml", "Commands.yml", "wk.yml"];

pub fn dir<P>(dir_path: P) -> Result<PathBuf, Error>
where
  P: AsRef<Path>,
{
  dir_with_patterns(dir_path, None)
}

pub fn dir_with_patterns<P>(dir_path: P, patterns: Option<Vec<&str>>) -> Result<PathBuf, Error>
where
  P: AsRef<Path>,
{
  let dir_path_ref = dir_path.as_ref();
  let patterns = patterns.unwrap_or(FILES.to_vec());

  let mut dir_pathbuf = PathBuf::new().join(&dir_path_ref);

  if !dir_pathbuf.is_absolute() {
    if let Ok(cwd) = std::env::current_dir() {
      dir_pathbuf = PathBuf::new().join(cwd).join(dir_pathbuf);
    }
  }

  if !dir_pathbuf.is_dir() {
    let d = dir_pathbuf.display();
    return Err(Error::Import(format!("\"{}\" is not a directory", d)));
  }

  let dirpath = dir_pathbuf.as_path();
  let readdir = std::fs::read_dir(&dirpath)?;

  let items: Vec<PathBuf> = patterns
    .iter()
    .map(|pattern| PathBuf::new().join(&dirpath).join(&pattern))
    .collect();

  let mut it = readdir.into_iter();
  while let Some(item) = it.next() {
    if let Ok(entry) = item {
      let entry_path = entry.path();
      if items.contains(&entry_path) {
        return Ok(entry_path);
      }
    }
  }

  Err(Error::Import("No commands found.".to_string()))
}
