#![allow(dead_code)]

mod command;
mod concurrent;
mod context;
mod error;
mod executor;
mod importer;
mod test;
mod utils;

use crate::{
  error::Error,
  context::Context,
  importer::lookup_and_load,
};
use futures::executor::block_on;

async fn run() -> Result<(), Error> {
  let dir_path = std::env::current_dir()?;
  let context = lookup_and_load(dir_path.as_path())?;

  let args: Vec<String> = std::env::args()
  .enumerate()
  .filter_map(|a| {
    if a.0 != 0 {
      Some(a.1)
    } else {
      None
    }
  })
  .collect();

  if args.len() > 0 {
    context.run(&args[0], None).await?;
  } else {
    for task in context.tasks.keys() {
      println!("{}", task);
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
