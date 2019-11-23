use crate::utils::path::*;
use crate::importer::{load, CommandImported};
use crate::utils;
use crate::command::CommandBuilder;

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
fn parse_file() -> Result<(), Box<dyn std::error::Error>> {
  let path: PathBuf = Path::new("./")
    .join("tmp")
    .join("Commands.yml")
    .normalize();

  let mut tasks = load(&path)?;
  // println!("{:#?}", tasks);

  if let Some(imported) = tasks.remove("build") {
    match imported {
      CommandImported::Command(build) => {
        build.into_command();
      },
      _ => {}
    }
  }

  // task::Task::new().with_name("Coucou").with_cwd(&path);

  // let task: CommandBuilder = "echo Hello World".parse().unwrap();
  // println!("{:#?}", task);

  Ok(())
}