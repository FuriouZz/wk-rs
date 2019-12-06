use crate::{
  command::CommandBuilder, concurrent::Concurrent, context::Context, error::Error,
  utils::fs::Reader,
};
use serde::Deserialize;
use serde_yaml;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

const FILES: [&'static str; 2] = ["commands.yml", "Commands.yml"];

#[derive(Deserialize, Debug)]
struct CommandsFile {
  extends: Option<Vec<PathBuf>>,
  commands: HashMap<String, CommandFileDescription>,
  variables: Option<HashMap<String, Primitive>>,
  environments: Option<HashMap<String, Primitive>>,
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
  cwd: Option<PathBuf>,
  args: Option<Vec<String>>,
  shell: Option<PathBuf>,
  hidden: Option<bool>,
  command: String,
  depends: Option<Vec<String>>,
  variables: Option<HashMap<String, Primitive>>,
  environments: Option<HashMap<String, Primitive>>,
  description: Option<String>,
}

#[derive(Deserialize, Debug)]
struct ExtendedCommandDescription {
  cwd: Option<PathBuf>,
  args: Option<Vec<String>>,
  shell: Option<PathBuf>,
  hidden: Option<bool>,
  extend: String,
  depends: Option<Vec<String>>,
  variables: Option<HashMap<String, Primitive>>,
  description: Option<String>,
  environments: Option<HashMap<String, Primitive>>,
}

#[derive(Deserialize, Debug)]
struct ConcurrentDescription {
  hidden: Option<bool>,
  depends: Option<Vec<String>>,
  commands: Vec<String>,
  variables: Option<HashMap<String, Primitive>>,
  description: Option<String>,
  environments: Option<HashMap<String, Primitive>>,
}

pub struct ExtendedCommand {
  extend: CommandBuilder,
  desc: ExtendedCommandDescription,
}

// Later implementation for different variable types
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
enum Primitive {
  S(String),
  B(bool),
  F(f64),
  I(i32),
}

#[derive(Debug, Clone)]
pub enum CommandImported {
  Command(CommandBuilder),
  Concurrent(Concurrent),
}

impl From<CommandDescription> for CommandBuilder {
  fn from(value: CommandDescription) -> Self {
    let mut task = CommandBuilder::new();
    task.with_command(value.command).with_cwd(value.cwd);

    if let Some(args) = value.args {
      task.with_args(args);
    }
    if let Some(shell) = value.shell {
      task.with_shell(shell);
    }
    if let Some(hidden) = value.hidden {
      task.with_hidden(hidden);
    }
    if let Some(dependencies) = value.depends {
      task.with_dependencies(dependencies);
    }
    if let Some(variables) = value.variables {
      task.with_variables(ptos(variables));
    }
    if let Some(environments) = value.environments {
      task.with_environments(ptos(environments));
    }
    if let Some(description) = value.description {
      task.with_description(description);
    }

    return task;
  }
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
      concurrent.with_variables(ptos(variables));
    }
    if let Some(environments) = value.environments {
      concurrent.with_environments(ptos(environments));
    }

    return concurrent;
  }
}

impl From<ExtendedCommand> for CommandBuilder {
  fn from(value: ExtendedCommand) -> Self {
    let mut task = value.extend;
    task.with_cwd(value.desc.cwd);

    if let Some(args) = value.desc.args {
      task.override_args(args);
    }
    if let Some(shell) = value.desc.shell {
      task.with_shell(shell);
    }
    if let Some(hidden) = value.desc.hidden {
      task.with_hidden(hidden);
    }
    if let Some(dependencies) = value.desc.depends {
      task.with_dependencies(dependencies);
    }
    if let Some(variables) = value.desc.variables {
      task.with_variables(ptos(variables));
    }
    if let Some(environments) = value.desc.environments {
      task.with_environments(ptos(environments));
    }
    if let Some(description) = value.desc.description {
      task.with_description(description);
    }

    return task;
  }
}

impl From<Primitive> for String {
  fn from(value: Primitive) -> Self {
    match value {
      Primitive::S(s) => s,
      Primitive::F(f) => f.to_string(),
      Primitive::I(i) => i.to_string(),
      Primitive::B(b) => b.to_string(),
    }
  }
}

fn ptos(map: HashMap<String, Primitive>) -> HashMap<String, String> {
  let mut h: HashMap<String, String> = HashMap::new();
  for item in map {
    h.insert(item.0, item.1.into());
  }
  return h;
}

pub fn load<P>(path: P) -> Result<Context, Error>
where
  P: AsRef<Path> + Copy,
{
  let content = Reader::text(path)?;
  let file: CommandsFile = serde_yaml::from_str(content.as_str())?;

  let mut source = PathBuf::new();
  source.push(&path);

  let mut tasks: HashMap<String, CommandImported> = HashMap::new();
  let mut extends: Vec<(String, ExtendedCommandDescription)> = Vec::new();

  // Variables
  let variables = file.variables.unwrap_or(HashMap::new());

  // Environments
  let environments = file.environments.unwrap_or(HashMap::new());

  // Create tasks
  let mut commands = file.commands.into_iter();
  while let Some((key, value)) = commands.next() {
    let name: String = key;
    let command: CommandFileDescription = value;

    match command {
      CommandFileDescription::StringCommand(command) => {
        let mut task: CommandBuilder = command.as_str().parse::<CommandBuilder>()?;
        let vars = task.variables.clone();
        let envs = task.environments.clone();
        task
          .with_name(name.clone())
          .with_source(source.clone())
          .with_variables(ptos(variables.clone())) // Apply file variables
          .with_variables(vars) // Override variables with task
          .with_environments(ptos(environments.clone())) // Apply file environments
          .with_environments(envs); // Override environments with task
        tasks.insert(name.clone(), CommandImported::Command(task));
      }
      CommandFileDescription::Command(task_desc) => {
        let mut task: CommandBuilder = task_desc.into();
        let vars = task.variables.clone();
        let envs = task.environments.clone();
        task
          .with_name(name.clone())
          .with_source(source.clone())
          .with_variables(ptos(variables.clone())) // Apply file variables
          .with_variables(vars) // Override variables with task
          .with_environments(ptos(environments.clone())) // Apply file environments
          .with_environments(envs); // Override environments with task
        tasks.insert(name.clone(), CommandImported::Command(task));
      }
      CommandFileDescription::Concurrent(conc_desc) => {
        let mut conc: Concurrent = conc_desc.into();
        let vars = conc.variables.clone();
        let envs = conc.environments.clone();
        conc
          .with_name(name.clone())
          .with_source(source.clone())
          .with_variables(ptos(variables.clone())) // Apply file variables
          .with_variables(vars) // Override variables with task
          .with_environments(ptos(environments.clone())) // Apply file environments
          .with_environments(envs); // Override environments with task
        tasks.insert(name.clone(), CommandImported::Concurrent(conc));
      }
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
        let extend = ExtendedCommand {
          extend: (*task).clone(),
          desc: command,
        };

        let mut task: CommandBuilder = extend.into();
        task.with_name(name.clone());
        tasks.insert(name.clone(), CommandImported::Command(task));
      } else {
        return Err(Error::ImportError(format!(
          "Cannot extend {}",
          command.extend.clone()
        )));
      }
    } else {
      return Err(Error::ImportError(format!(
        "Cannot extend {}",
        command.extend.clone()
      )));
    }
  }

  // Return context
  let mut context = Context { tasks };

  if let Some(context_extends) = file.extends {
    for f in context_extends {
      let relative_path = source.parent().expect("Source has no parent");
      let ff = relative_path.join(f);
      let fpath = ff.as_path();

      if fpath != source.as_path() {
        let c = load(fpath)?;
        context.extend(&c);
      } else {
        return Err(Error::ImportError(format!("Cannot extend {:?}", fpath).to_string()));
      }
    }
  }

  Ok(context)
}

pub fn lookup_dir<P>(dir_path: P) -> Result<PathBuf, Error>
where
  P: AsRef<Path>,
{
  lookup(dir_path, None)
}

pub fn lookup<P>(dir_path: P, patterns: Option<Vec<&str>>) -> Result<PathBuf, Error>
where
  P: AsRef<Path>,
{
  let patterns = patterns.unwrap_or(FILES.to_vec());

  let dir_path = dir_path.as_ref();
  let mut dir_pathbuf = PathBuf::new().join(dir_path);

  if !dir_pathbuf.is_absolute() {
    if let Ok(cwd) = std::env::current_dir() {
      dir_pathbuf = PathBuf::new().join(cwd).join(dir_pathbuf);
    }
  }

  if !dir_pathbuf.is_dir() {
    let d = dir_pathbuf.display();
    return Err(Error::ImportError(format!("\"{}\" is not a directory", d)));
  }

  let dir_path = dir_pathbuf.as_path();
  let readdir = std::fs::read_dir(dir_path)?;

  let items: Vec<PathBuf> = patterns
    .iter()
    .map(|pattern| PathBuf::new().join(&dir_path).join(&pattern))
    .collect();

  let mut it = readdir.into_iter();
  while let Some(item) = it.next() {
    if let Ok(entry) = item {
      let entry_path = entry.path();
      if items.contains(&entry_path) {
        return Ok(entry_path);
      }
    }
  }

  Err(Error::ImportError("No commands found.".to_string()))
}

pub fn lookup_and_load<P>(dir_path: P) -> Result<Context, Error>
where
  P: AsRef<Path>,
{
  let path = lookup_dir(dir_path)?;
  load(path.as_path())
}
