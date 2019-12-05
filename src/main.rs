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
  importer::{load, lookup_dir},
};
use futures::executor::block_on;

async fn run(context: &Context) -> Result<(), Error> {
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
  let dir_path = std::env::current_dir()?;
  let res = lookup_dir(dir_path)?;
  let context = load(res.as_path())?;
  block_on(run(&context))
}
