mod importer;
mod command;
mod concurrent;
mod utils;
mod test;
mod error;
mod executor;
mod context;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  println!("{:#?}", args);
}