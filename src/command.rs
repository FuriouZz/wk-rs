use crate::utils::fs::{FileError, Reader};
use serde::Deserialize;
use std::path::{ Path, PathBuf };

// export interface Command {
//   source?: string;
//   conditions?: CommandCondition[];
//   variables?: Record<string, string>;
//   subcommands?: (string|Command)[]
// }

#[derive(Deserialize, Clone, Debug)]
pub struct CommandFile {
  commands: std::collections::HashMap<String, CommandSchema>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CommandSchema {
  command: String,
  cwd: Option<std::path::PathBuf>,
  name: Option<String>,
  args: Option<Vec<String>>,
  visible: Option<bool>,
  bin_path: Option<std::path::PathBuf>,
  depends_on: Option<Vec<String>>,
  description: Option<String>,
  // subcommands: Option<Vec<CommandFile>>,
}

impl From<CommandSchema> for Task {

  fn from(value: CommandSchema) -> Self {
    let mut task = Task::new()
    .command(value.command);

    if let Some(cwd) = value.cwd {
      task = task.cwd(cwd);
    }
    if let Some(name) = value.name {
      task = task.name(name);
    }
    if let Some(visible) = value.visible {
      if visible {
        task = task.visible(TaskVisibility::Visible);
      } else {
        task = task.visible(TaskVisibility::Hidden);
      }
    }
    if let Some(description) = value.description {
      task = task.description(description);
    }
    if let Some(dependencies) = value.depends_on {
      for value in dependencies.iter() {
        task = task.depend(value.clone());
      }
    }
    if let Some(args) = value.args {
      for value in args.iter() {
        task = task.param(value.clone());
      }
    }
    if let Some(bin_path) = value.bin_path {
      task = task.bin_path(bin_path);
    }

    return task;
  }

}

pub fn load< P>(path: P) -> Result<Vec<Task>, FileError>
where
  P: AsRef<std::path::Path>,
{
  let result = Reader::text(path)?;
  let file: CommandFile = toml::from_str(result.as_str()).unwrap();

  let tasks: Vec<Task> = file.commands.iter().map(|(key, value)| {
    let mut command = value.clone();

    if let None = command.name {
      command.name = key.clone().into();
    }

    let task: Task = command.into();
    // task.source(&path);
    return task;
  }).collect();

  Ok(tasks)
}

#[derive(Debug)]
pub enum TaskVisibility {
  Visible,
  Hidden
}

#[derive(Debug)]
pub struct Task {
  command: String,
  cwd: PathBuf,
  name: String,
  source: PathBuf,
  visible: TaskVisibility,
  bin_path: PathBuf,
  parameters: Vec<String>,
  description: Option<String>,
  dependencies: Vec<String>,
}

impl Task {
  pub fn new() -> Self {
    Self {
      command: "".to_owned(),
      cwd: Path::new("").to_owned(),
      name: "task".to_owned(),
      source: Path::new("").to_owned(),
      visible: TaskVisibility::Visible,
      bin_path: Path::new("").to_owned(),
      parameters: Vec::new(),
      description: None,
      dependencies: Vec::new(),
    }
  }

  pub fn name<S>(mut self, name: S) -> Self
  where
    S: AsRef<str>,
  {
    self.name = name.as_ref().to_owned();
    self
  }

  pub fn description<S>(mut self, description: S) -> Self
  where
    S: AsRef<str>,
  {
    self.description = Some(description.as_ref().to_owned());
    self
  }

  pub fn command<S>(mut self, command: S) -> Self
  where
    S: AsRef<str>,
  {
    self.command = command.as_ref().to_owned();
    self
  }

  pub fn cwd<S>(mut self, cwd: S) -> Self
  where
    S: AsRef<Path>,
  {
    self.cwd = cwd.as_ref().to_owned();
    self
  }

  pub fn source<S>(mut self, source: S) -> Self
  where
    S: AsRef<Path>,
  {
    self.source = source.as_ref().to_owned();
    self
  }

  pub fn bin_path<S>(mut self, bin_path: S) -> Self
  where
    S: AsRef<Path>,
  {
    self.bin_path = bin_path.as_ref().to_owned();
    self
  }

  pub fn visible(mut self, visible: TaskVisibility) -> Self {
    self.visible = visible;
    self
  }

  pub fn depend<S>(mut self, dependency: S) -> Self
  where
    S: AsRef<str>,
  {
    self.dependencies.push(dependency.as_ref().to_owned());
    self
  }

  pub fn param<S>(mut self, parameter: S) -> Self
  where
    S: AsRef<str>,
  {
    self.parameters.push(parameter.as_ref().to_owned());
    self
  }
}
