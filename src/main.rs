mod utils;
mod importer;
mod task;

use std::path::{Path, PathBuf};
use utils::path::*;

fn main() -> Result<(), utils::fs::FileError> {
    let path: PathBuf = Path::new("./")
        .join("tmp")
        .join("Commands.toml")
        .normalize();

    let tasks = importer::load(&path)?;
    println!("{:#?}", tasks);

    task::Task::new()
    .name(String::from("Coucou"))
    .cwd(&path);

    let task: task::Task = "echo Hello World".parse().unwrap();
    println!("{:#?}", task);
    // // let result = Reader::toml_value(path)?;
    // println!("{:?}", result);

    // let command: CommandFile = toml::from_str(result.as_str()).unwrap();
    // println!("{:?}", command);

    Ok(())
}
