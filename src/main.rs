mod importer;
mod task;
mod utils;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  println!("{:#?}", args);
}

#[cfg(test)]
mod tests {
  use std::path::{Path, PathBuf};
  use super::utils::path::*;
  use super::{ utils };
  use super::*;

  #[test]
  fn parse_file() -> Result<(), utils::fs::FileError> {
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
}
