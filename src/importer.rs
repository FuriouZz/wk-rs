use crate::utils::fs::{Reader};
use crate::task::Task;
use crate::concurrent::Concurrent;
use serde_yaml;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct CommandsFile {
  extends: Option<Vec<String>>,
  variables: Option<std::collections::HashMap<String, String>>,
  commands: std::collections::HashMap<String, CommandDescription>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum CommandDescription {
  Command(String),
  Task(TaskDescription),
  ExtendedTask(ExtendedTaskDescription),
  Concurrent(ConcurrentDescription),
}

#[derive(Deserialize, Debug)]
struct TaskDescription {
  cwd: Option<std::path::PathBuf>,
  args: Option<Vec<String>>,
  hidden: Option<bool>,
  command: String,
  depends: Option<Vec<String>>,
  variables: Option<std::collections::HashMap<String, String>>,
  description: Option<String>,
}

impl From<TaskDescription> for Task {
  fn from(value: TaskDescription) -> Self {
    let mut task = Task::new().with_command(value.command);
    task = task.with_cwd(value.cwd);

    if let Some(args) = value.args {
      task = task.with_args(args);
    }
    if let Some(hidden) = value.hidden {
      task = task.with_hidden(hidden);
    }
    if let Some(dependencies) = value.depends {
      task = task.with_dependencies(dependencies);
    }
    if let Some(variables) = value.variables {
      task = task.with_variables(variables);
    }
    if let Some(description) = value.description {
      task = task.with_description(description);
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
    concurrent = concurrent.with_commands(value.commands);

    if let Some(hidden) = value.hidden {
      concurrent = concurrent.with_hidden(hidden);
    }
    if let Some(dependencies) = value.depends {
      concurrent = concurrent.with_dependencies(dependencies);
    }
    if let Some(description) = value.description {
      concurrent = concurrent.with_description(description);
    }
    if let Some(variables) = value.variables {
      concurrent = concurrent.with_variables(variables);
    }

    return concurrent;
  }
}

#[derive(Deserialize, Debug)]
struct ExtendedTaskDescription {
  cwd: Option<std::path::PathBuf>,
  args: Option<Vec<String>>,
  hidden: Option<bool>,
  extend: String,
  depends: Option<Vec<String>>,
  variables: Option<std::collections::HashMap<String, String>>,
  description: Option<String>,
}

pub struct ExtendedTask<'a> {
  extend: &'a Task,
  desc: ExtendedTaskDescription
}

impl<'a> From<ExtendedTask<'a>> for Task {
  fn from(value: ExtendedTask) -> Self {
    let mut task = value.extend.clone();
    task = task.with_cwd(value.desc.cwd);

    if let Some(args) = value.desc.args {
      task = task.with_args(args);
    }
    if let Some(hidden) = value.desc.hidden {
      task = task.with_hidden(hidden);
    }
    if let Some(dependencies) = value.desc.depends {
      task = task.with_dependencies(dependencies);
    }
    if let Some(variables) = value.desc.variables {
      task = task.with_variables(variables);
    }
    if let Some(description) = value.desc.description {
      task = task.with_description(description);
    }

    return task;
  }
}

#[derive(Debug)]
pub enum Command {
  Task(Task),
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
pub fn load<P>(path: P) -> Result<std::collections::HashMap<String, Command>, Box<dyn std::error::Error>>
where
  P: AsRef<std::path::Path> + Copy,
{
  let content = Reader::text(path)?;
  let file: CommandsFile = serde_yaml::from_str(content.as_str())?;

  let mut source = std::path::PathBuf::new();
  source.push(&path);

  let mut tasks: std::collections::HashMap<String, Command> = std::collections::HashMap::new();
  let mut extends: Vec<(String, ExtendedTaskDescription)> = Vec::new();

  // Create tasks
  let mut commands = file.commands.into_iter();
  while let Some((key, value)) = commands.next() {
    let name: String = key;
    let command: CommandDescription = value;

    match command {
      CommandDescription::Command(command) => {
        let mut task: Task = command.as_str().parse::<Task>()?;
        task = task.with_name(name.clone()).with_source(source.clone());
        tasks.insert(name.clone(), Command::Task(task));
      },
      CommandDescription::Task(task_desc) => {
        let mut task: Task = task_desc.into();
        task = task.with_name(name.clone()).with_source(source.clone());
        tasks.insert(name.clone(), Command::Task(task));
      },
      CommandDescription::Concurrent(conc_desc) => {
        let mut conc: Concurrent = conc_desc.into();
        conc = conc.with_name(name.clone()).with_source(source.clone());
        tasks.insert(name.clone(), Command::Concurrent(conc));
      },
      CommandDescription::ExtendedTask(extd_desc) => {
        extends.push((name, extd_desc));
      }
    }
  }

  // Create extended task
  for extd in extends {
    let name = extd.0;
    let command = extd.1;

    if let Some(cmd) = tasks.get(&command.extend) {
      if let Command::Task(task) = cmd {
        let extend = ExtendedTask {
          extend: &task,
          desc: command
        };

        let mut task: Task = extend.into();
        task = task.with_name(name.clone());
        tasks.insert(name.clone(), Command::Task(task));
      } else {
        println!("Task \"{}\" Cannot extend the concurrent task \"{}\".", name, command.extend);
        // return Err(Box::new("Cannot extend a concurrent task."));
      }
    }
  }

  Ok(tasks)
}