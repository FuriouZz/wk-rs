#[cfg(test)]
mod tests {
  use crate:: {
    utils::path::PathExt,
    importer::{
      CommandImported, load, lookup_dir
    },
    executor::Executor,
  };
  use std::{
    collections::HashMap,
    path::{
      Path, PathBuf,
    }
  };

  fn run(name: &str, executor: &Executor, tasks: &HashMap<String, CommandImported>) {
    if let Some(imported) = tasks.get(name) {
      match imported {
        CommandImported::Command(builder) => {
          let mut r = builder.to_command();
          executor.spawn(async {
            r.execute().await;
          });
        },
        _ => {}
      }
    }
  }

  async fn run_async(name: &str, tasks: &HashMap<String, CommandImported>) {
    if let Some(imported) = tasks.get(name) {
      match imported {
        CommandImported::Command(builder) => {
          builder.to_command().execute().await;
        },
        _ => {}
      }
    }
  }

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
    let path: PathBuf = Path::new("./")
      .join("tmp")
      .join("simple.yml")
      .normalize();

    if let Ok(context) = load(&path) {
      context.run("how").await;
    }
  }
}