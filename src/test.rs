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
            r.execute();
            r.await;
          });
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
    let path: PathBuf = Path::new("./")
      .join("tmp")
      .join("simple.yml")
      .normalize();

    let tasks = load(&path)?;

    let e = Executor::new();
    run("echo", &e, &tasks);
    run("john", &e, &tasks);
    e.run();

    Ok(())
  }
}