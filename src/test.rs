use crate::utils::path::*;
use crate::importer;
use crate::utils;
use crate::task;

#[cfg(test)]
use std::path::{Path, PathBuf};

const PATHS: [&'static str; 2] = ["commands.toml", "Commands.toml"];

#[test]
fn lookup() {
  let mut dir_path = std::env::current_dir().unwrap();
  dir_path.push("tmp");
  let res = utils::fs::fetch(dir_path, &PATHS[1]);
  println!("{:?}", res);
}

#[test]
fn parse_file2() -> Result<(), Box<dyn std::error::Error>> {
  let path: PathBuf = Path::new("./")
    .join("tmp")
    .join("tmp")
    .join("Commands.yml")
    .normalize();

  let tasks = importer::load(&path)?;
  println!("{:#?}", tasks);

  // task::Task::new().with_name("Coucou").with_cwd(&path);

  let task: task::Task = "echo Hello World".parse().unwrap();
  println!("{:#?}", task);

  Ok(())
}
