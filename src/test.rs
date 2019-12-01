use crate::utils::path::*;
use crate::importer;
use crate::utils;
use crate::command::CommandBuilder;
use crate::runner::Runner;
use crate::executor::Executor;
use futures::executor::block_on;
use futures::future::join;
use futures::future::FutureExt;

fn run(name: &str, tasks: &std::collections::HashMap<String, importer::CommandImported>) -> Option<Runner> {

  if let Some(imported) = tasks.get(name) {
    match imported {
      importer::CommandImported::Command(builder) => {
        let mut r = Runner::new();
        r.execute(&builder);
        return Some(r);
      },
      _ => {}
    }
  }

  None
}

#[cfg(test)]
use std::path::{Path, PathBuf};

#[test]
fn lookup() {
  let mut dir_path = std::env::current_dir().unwrap();
  dir_path.push("tmp");
  let res = importer::lookup_dir(dir_path);
  println!("{:?}", res);
}

#[test]
fn parse_file() -> Result<(), Box<dyn std::error::Error>> {
  let path: PathBuf = Path::new("./")
    .join("tmp")
    .join("simple.yml")
    .normalize();

  let tasks = importer::load(&path)?;
  // println!("{:#?}", tasks);

  let e = Executor::new();

  let f0 = run("echo", &tasks);
  let f1 = run("john", &tasks);

  if let Some(r) = f0 {
    e.spawn(async {
      r.await;
    })
  }
  if let Some(r) = f1 {
    e.spawn(async {
      r.await;
    })
  }

  e.run();
  // e.spawn(f0);

  // let pair = join(f0, f1);
  // block_on(pair);
  // block_on(f0);

  // if let Some(imported) = tasks.get("echo") {
  //   match imported {
  //     importer::CommandImported::Command(builder) => {
  //       let hello_world_fut = async {
  //         let f = run(builder);
  //         println!("Cool1");
  //         f.await;
  //         println!("Cool2");
  //       };

  //       println!("Cool0");
  //       let res = block_on(hello_world_fut);
  //       println!("Cool3");

  //       // println!("{:?}", f);

  //       // let cmd = build.into_command();
  //       // println!("{:?}", cmd);

  //       // // let args = cmd.args.as_slice();
  //       // // let first = &args[0];
  //       // // let ps = &args[1..];
  //       // // let ps = ps.to_vec();

  //       // let p = std::process::Command::new("cmd.exe")
  //       // .arg("/c")
  //       // .args(cmd.args)
  //       // .spawn()
  //       // .unwrap();

  //       // println!("{:?}", p);
  //     },
  //     _ => {}
  //   }
  // }

  // task::Task::new().with_name("Coucou").with_cwd(&path);

  // let task: CommandBuilder = "echo Hello World".parse().unwrap();
  // println!("{:#?}", task);

  Ok(())
}