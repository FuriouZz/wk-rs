use crate::utils::fs::{Reader};
use crate::command::Command;
use crate::concurrent::Concurrent;
use serde_yaml;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct CommandsFile {
  extends: Option<Vec<String>>,
  variables: Option<std::collections::HashMap<String, String>>,
  commands: std::collections::HashMap<String, CommandFileDescription>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum CommandFileDescription {
  StringCommand(String),
  Command(CommandDescription),
  ExtendedCommand(ExtendedCommandDescription),
  Concurrent(ConcurrentDescription),
}

#[derive(Deserialize, Debug)]
struct CommandDescription {
  cwd: Option<std::path::PathBuf>,
  args: Option<Vec<String>>,
  hidden: Option<bool>,
  command: String,
  depends: Option<Vec<String>>,
  variables: Option<std::collections::HashMap<String, String>>,
  description: Option<String>,
}

impl From<CommandDescription> for Command {
  fn from(value: CommandDescription) -> Self {
    let mut task = Command::new();
    task.with_command(value.command).with_cwd(value.cwd);

    if let Some(args) = value.args {
      task.with_args(args);
    }
    if let Some(hidden) = value.hidden {
      task.with_hidden(hidden);
    }
    if let Some(dependencies) = value.depends {
      task.with_dependencies(dependencies);
    }
    if let Some(variables) = value.variables {
      task.with_variables(variables);
    }
    if let Some(description) = value.description {
      task.with_description(description);
    }

    return task;
  }
}

#[derive(Deserialize, Debug)]
struct ConcurrentDescription {
  hidden: Option<bool>,
  depends: Option<Vec<String>>,
  commands: Vec<String>,
  variables: Option<std::collections::HashMap<String, String>>,
  description: Option<String>,
}

impl From<ConcurrentDescription> for Concurrent {
  fn from(value: ConcurrentDescription) -> Self {
    let mut concurrent = Concurrent::new();
    concurrent.with_commands(value.commands);

    if let Some(hidden) = value.hidden {
      concurrent.with_hidden(hidden);
    }
    if let Some(dependencies) = value.depends {
      concurrent.with_dependencies(dependencies);
    }
    if let Some(description) = value.description {
      concurrent.with_description(description);
    }
    if let Some(variables) = value.variables {
      concurrent.with_variables(variables);
    }

    return concurrent;
  }
}

#[derive(Deserialize, Debug)]
struct ExtendedCommandDescription {
  cwd: Option<std::path::PathBuf>,
  args: Option<Vec<String>>,
  hidden: Option<bool>,
  extend: String,
  depends: Option<Vec<String>>,
  variables: Option<std::collections::HashMap<String, String>>,
  description: Option<String>,
}

pub struct ExtendedTask<'a> {
  extend: &'a Command,
  desc: ExtendedCommandDescription
}

impl<'a> From<ExtendedTask<'a>> for Command {
  fn from(value: ExtendedTask) -> Self {
    let mut task = value.extend.clone();
    task.with_cwd(value.desc.cwd);

    if let Some(args) = value.desc.args {
      task.with_args(args);
    }
    if let Some(hidden) = value.desc.hidden {
      task.with_hidden(hidden);
    }
    if let Some(dependencies) = value.desc.depends {
      task.with_dependencies(dependencies);
    }
    if let Some(variables) = value.desc.variables {
      task.with_variables(variables);
    }
    if let Some(description) = value.desc.description {
      task.with_description(description);
    }

    return task;
  }
}

#[derive(Debug)]
pub enum CommandImported {
  Command(Command),
  Concurrent(Concurrent)
}

// Later implementation for different variable types
// #[derive(Deserialize, Debug)]
// #[serde(untagged)]
// enum Primitive {
//   S(String),
//   B(bool),
//   F64(f64),
//   I(i64),
// }

#[allow(dead_code)]
pub fn load<P>(path: P) -> Result<std::collections::HashMap<String, CommandImported>, Box<dyn std::error::Error>>
where
  P: AsRef<std::path::Path> + Copy,
{
  let content = Reader::text(path)?;
  let file: CommandsFile = serde_yaml::from_str(content.as_str())?;

  let mut source = std::path::PathBuf::new();
  source.push(&path);

  let mut tasks: std::collections::HashMap<String, CommandImported> = std::collections::HashMap::new();
  let mut extends: Vec<(String, ExtendedCommandDescription)> = Vec::new();

  // Create tasks
  let mut commands = file.commands.into_iter();
  while let Some((key, value)) = commands.next() {
    let name: String = key;
    let command: CommandFileDescription = value;

    match command {
      CommandFileDescription::StringCommand(command) => {
        let mut task: Command = command.as_str().parse::<Command>()?;
        task.with_name(name.clone()).with_source(source.clone());
        tasks.insert(name.clone(), CommandImported::Command(task));
      },
      CommandFileDescription::Command(task_desc) => {
        let mut task: Command = task_desc.into();
        task.with_name(name.clone()).with_source(source.clone());
        tasks.insert(name.clone(), CommandImported::Command(task));
      },
      CommandFileDescription::Concurrent(conc_desc) => {
        let mut conc: Concurrent = conc_desc.into();
        conc.with_name(name.clone()).with_source(source.clone());
        tasks.insert(name.clone(), CommandImported::Concurrent(conc));
      },
      CommandFileDescription::ExtendedCommand(extd_desc) => {
        extends.push((name, extd_desc));
      }
    }
  }

  // Create extended task
  for extd in extends {
    let name = extd.0;
    let command = extd.1;

    if let Some(cmd) = tasks.get(&command.extend) {
      if let CommandImported::Command(task) = cmd {
        let extend = ExtendedTask {
          extend: &task,
          desc: command
        };

        let mut task: Command = extend.into();
        task.with_name(name.clone());
        tasks.insert(name.clone(), CommandImported::Command(task));
      } else {
        println!("Task \"{}\" Cannot extend the concurrent task \"{}\".", name, command.extend);
        // return Err(Box::new("Cannot extend a concurrent task."));
      }
    }
  }

  Ok(tasks)
}