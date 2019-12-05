#[cfg(test)]
mod tests {
  use crate::{
    executor::Executor,
    importer::{load, lookup_dir, CommandImported},
    utils::path::PathExt,
  };
  use std::{
    collections::HashMap,
    path::{Path, PathBuf},
  };

  #[test]
  fn lookup() {
    let mut dir_path = std::env::current_dir().unwrap();
    dir_path.push("tmp");
    let res = lookup_dir(dir_path);
    println!("{:?}", res);
  }

  #[test]
  fn parse_file() -> Result<(), Box<dyn std::error::Error>> {
    futures::executor::block_on(parse_file_async());
    Ok(())
  }

  async fn parse_file_async() {
    let path: PathBuf = Path::new("./").join("tmp").join("simple.yml").normalize();

    if let Ok(context) = load(&path) {
      context.run("how", None).await;
      // context.run("ls", None).await;
    }
  }
}
