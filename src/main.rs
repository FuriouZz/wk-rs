#![allow(dead_code)]

mod command;
mod concurrent;
mod context;
mod error;
mod executor;
mod importer;
mod test;
mod utils;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  println!("{:#?}", args);
}
