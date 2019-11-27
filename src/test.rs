use crate::utils::path::*;
use crate::importer;
use crate::utils;
use crate::command::CommandBuilder;

#[cfg(test)]
use std::path::{Path, PathBuf};

#[test]
fn lookup() {
  let mut dir_path = std::env::current_dir().unwrap();
  dir_path.push("tmp");
  let res = importer::lookup_dir(dir_path);
  println!("{:?}", res);
}

#[test]
fn parse_file() -> Result<(), Box<dyn std::error::Error>> {
  let path: PathBuf = Path::new("./")
    .join("tmp")
    .join("Commands.yml")
    .normalize();

  let tasks = importer::load(&path)?;
  println!("{:#?}", tasks);

  // if let Some(imported) = tasks.get("echo") {
  //   match imported {
  //     importer::CommandImported::Command(build) => {
  //       let cmd = build.into_command();
  //       println!("{:?}", cmd);
  //     },
  //     _ => {}
  //   }
  // }

  // task::Task::new().with_name("Coucou").with_cwd(&path);

  // let task: CommandBuilder = "echo Hello World".parse().unwrap();
  // println!("{:#?}", task);

  Ok(())
}