mod utils;
mod command;

use std::path::{Path, PathBuf};
use utils::path::*;

fn main() -> Result<(), utils::fs::FileError> {
    let path: PathBuf = Path::new("./")
        .join("tmp")
        .join("Commands.toml")
        .normalize();

    let tasks = command::load(&path)?;
    println!("{:#?}", tasks);

    command::Task::new().cwd(PathBuf::from(&path));

    // // let result = Reader::toml_value(path)?;
    // println!("{:?}", result);

    // let command: CommandFile = toml::from_str(result.as_str()).unwrap();
    // println!("{:?}", command);

    Ok(())
}
