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
  importer::lookup_and_load,
  utils::argv,
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

  let argv = argv::parse(args.iter());
  let vars = argv::extract_vars(&argv);

  if let Some(task) = argv.get("0") {
    context.run(task, Some(&vars)).await?;
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
