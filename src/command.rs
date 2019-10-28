use crate::utils::fs::{FileError, Reader};
use serde::Deserialize;
use std::borrow::Cow;
use std::path::Path;

// export interface Command {
//   source?: string;
//   cwd?: string;
//   binPath?: string;
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
  visible: Option<bool>,
  args: Option<Vec<String>>,
  depends_on: Option<Vec<String>>,
  description: Option<String>,
  // subcommands: Option<Vec<CommandFile>>,
}

impl<'a> From<CommandSchema> for Task<'a> {

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

    return task;
  }

}

pub fn load<'a, P>(path: P) -> Result<Vec<Task<'a>>, FileError>
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
pub struct Task<'a> {
  command: Cow<'a, str>,
  cwd: Cow<'a, Path>,
  name: Cow<'a, str>,
  visible: TaskVisibility,
  description: Option<Cow<'a, str>>,
  dependencies: Vec<Cow<'a, str>>,
  parameters: Vec<Cow<'a, str>>,
}

impl<'a> Task<'a> {
  pub fn new() -> Self {
    Self {
      command: Cow::Borrowed(""),
      cwd: Cow::Borrowed(Path::new("")),
      name: Cow::Borrowed("task"),
      visible: TaskVisibility::Visible,
      description: None,
      dependencies: Vec::new(),
      parameters: Vec::new(),
    }
  }

  pub fn name<S>(mut self, name: S) -> Self
  where
    S: Into<Cow<'a, str>>,
  {
    self.name = name.into();
    self
  }

  pub fn description<S>(mut self, description: S) -> Self
  where
    S: Into<Cow<'a, str>>,
  {
    self.description = Some(description.into());
    self
  }

  pub fn command<S>(mut self, command: S) -> Self
  where
    S: Into<Cow<'a, str>>,
  {
    self.command = command.into();
    self
  }

  pub fn cwd<S>(mut self, cwd: S) -> Self
  where
    S: Into<Cow<'a, Path>>,
  {
    self.cwd = cwd.into();
    self
  }

  pub fn visible(mut self, visible: TaskVisibility) -> Self {
    self.visible = visible;
    self
  }

  pub fn depend<S>(mut self, dependency: S) -> Self
  where
    S: Into<Cow<'a, str>>,
  {
    self.dependencies.push(dependency.into());
    self
  }

  pub fn param<S>(mut self, parameter: S) -> Self
  where
    S: Into<Cow<'a, str>>,
  {
    self.parameters.push(parameter.into());
    self
  }
}
