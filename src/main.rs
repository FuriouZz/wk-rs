mod importer;
mod command;
mod concurrent;
mod utils;
mod test;
mod error;
mod runner;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  println!("{:#?}", args);
}

