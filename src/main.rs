mod importer;
mod command;
mod concurrent;
mod utils;
mod test;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  println!("{:#?}", args);
}

