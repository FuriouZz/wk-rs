use crate::importer;
use crate::importer2;
use crate::task;
use crate::utils;

use super::utils::path::*;
use super::*;
#[cfg(test)]
use std::path::{Path, PathBuf};

const PATHS: [&'static str; 2] = ["commands.toml", "Commands.toml"];

#[test]
fn lookup() {
  let mut dir_path = std::env::current_dir().unwrap();
  dir_path.push("tmp");
  utils::fs::fetch(dir_path, &PATHS[1]);
}

#[test]
fn parse_file() -> Result<(), Box<dyn std::error::Error>> {
  let path: PathBuf = Path::new("./")
    .join("tmp")
    .join("Commands.toml")
    .normalize();

  let tasks = importer::load(&path)?;
  println!("{:#?}", tasks);

  task::Task::new().with_name("Coucou").with_cwd(&path);

  let task: task::Task = "echo Hello World".parse().unwrap();
  println!("{:#?}", task);
  // // let result = Reader::toml_value(path)?;
  // println!("{:?}", result);

  // let command: CommandFile = toml::from_str(result.as_str()).unwrap();
  // println!("{:?}", command);

  Ok(())
}

#[test]
fn parse_file2() -> Result<(), Box<dyn std::error::Error>> {
  let path: PathBuf = Path::new("./")
    .join("tmp")
    .join("tmp")
    .join("Commands.yml")
    .normalize();

  let tasks = importer2::load(&path)?;
  println!("{:#?}", tasks);

  // task::Task::new().with_name("Coucou").with_cwd(&path);

  // let task: task::Task = "echo Hello World".parse().unwrap();
  // println!("{:#?}", task);
  // // let result = Reader::toml_value(path)?;
  // println!("{:?}", result);

  // let command: CommandFile = toml::from_str(result.as_str()).unwrap();
  // println!("{:?}", command);

  Ok(())
}
