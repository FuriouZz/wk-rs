#![allow(dead_code)]

mod command;
mod concurrent;
mod context;
mod error;
mod importer;
mod test;
mod utils;

use crate::{error::Error, importer::lookup_and_load, utils::argv};
use futures::executor::block_on;

async fn run() -> Result<(), Error> {
  let dir_path = std::env::current_dir()?;
  let context = lookup_and_load(dir_path.as_path())?;

  let (params, vars) = argv::extract_vars_from_args(std::env::args());

  if params.len() > 0 {
    context.run(&params[0], Some(&vars)).await?;
  } else {
    println!("Task availables");
    for task in context.tasks.keys() {
      println!("  {}", task);
    }
  }

  Ok(())
}

fn main() -> Result<(), Error> {
  if let Err(e) = block_on(run()) {
    println!("{:#}", e);
  }
  Ok(())
}
