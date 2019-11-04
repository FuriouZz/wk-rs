use crate::task::{Task, TaskVisibility};
use crate::utils::fs::{FileError, Reader};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

// export interface Command {
//   conditions?: CommandCondition[];
// }
//
//
// concurrents: FileConcurrentRecord;
// importGlobals?: boolean;
// importPackage?: boolean;
// imports?: string[];

#[derive(Deserialize, Debug)]
pub struct CommandFile {
  commands: HashMap<String, TaskDescription>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TaskDescription {
  command: String,
  cwd: Option<std::path::PathBuf>,
  args: Option<Vec<String>>,
  visible: Option<bool>,
  bin_path: Option<std::path::PathBuf>,
  variables: Option<HashMap<String, String>>,
  depends_on: Option<Vec<String>>,
  description: Option<String>,
  subcommands: Option<Vec<SubTaskDescription>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SubTaskDescription {
  command: Option<String>,
  cwd: Option<std::path::PathBuf>,
  name: String,
  args: Option<Vec<String>>,
  visible: Option<bool>,
  bin_path: Option<std::path::PathBuf>,
  variables: Option<HashMap<String, String>>,
  depends_on: Option<Vec<String>>,
  description: Option<String>,
}

impl From<TaskDescription> for Task {
  fn from(value: TaskDescription) -> Self {
    let mut task = Task::new().with_command(value.command);

    if let Some(cwd) = value.cwd {
      task = task.with_cwd(cwd);
    }
    if let Some(visible) = value.visible {
      if visible {
        task = task.with_visible(TaskVisibility::Visible);
      } else {
        task = task.with_visible(TaskVisibility::Hidden);
      }
    }
    if let Some(description) = value.description {
      task = task.with_description(description);
    }
    if let Some(dependencies) = value.depends_on {
      let mut iterator = dependencies.into_iter();
      for value in iterator.next() {
        task = task.with_dependency(value);
      }
    }
    if let Some(args) = value.args {
      let mut iterator = args.into_iter();
      for value in iterator.next() {
        task = task.with_parameter(value);
      }
    }
    if let Some(bin_path) = value.bin_path {
      task = task.with_bin_path(bin_path);
    }
    if let Some(variables) = value.variables {
      task = task.with_variables(variables);
    }

    return task;
  }
}

pub fn load<P>(path: P) -> Result<Vec<Task>, FileError>
where
  P: AsRef<Path> + Copy,
{
  let result = Reader::text(path)?;
  let file: CommandFile = toml::from_str(result.as_str()).unwrap();

  let mut tasks = Vec::new();
  let mut iterator = file.commands.into_iter();
  while let Some((key, value)) = iterator.next() {
    let cmd_name: String = key;
    let mut cmd: TaskDescription = value;

    let mut source = PathBuf::new();
    source.push(&path);

    let subcmds = std::mem::replace(&mut cmd.subcommands, None);

    if let Some(subs) = subcmds {
      let mut iterator = subs.into_iter();
      while let Some(sub) = iterator.next() {
        let mut sub: SubTaskDescription = sub;
        let mut subcmd: TaskDescription = cmd.clone();

        if let Some(c) = sub.command {
          subcmd.command = c;
        }

        let variables = std::mem::replace(&mut sub.variables, None);
        if let Some(vars) = variables {
          subcmd.variables = subcmd.variables.map(|mut v| {
            v.extend(vars);
            v
          })
        }

        subcmd.cwd = sub.cwd.or(subcmd.cwd);
        subcmd.args = sub.args.or(subcmd.args);
        subcmd.visible = sub.visible.or(subcmd.visible);
        subcmd.bin_path = sub.bin_path.or(subcmd.bin_path);
        subcmd.depends_on = sub.depends_on.or(subcmd.depends_on);
        subcmd.description = sub.description.or(subcmd.description);

        let subtask: Task = subcmd.into();
        tasks.push(subtask.with_name(sub.name).with_source(source.clone()));
      }
    }

    let task: Task = cmd.into();
    tasks.push(task.with_name(cmd_name).with_source(source));
  }

  Ok(tasks)
}
