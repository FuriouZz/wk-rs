use crate::{
  command::CommandBuilder, concurrent::Concurrent, context::Context, error::Error,
  utils::fs::Reader,
};
use serde::Deserialize;
use serde_yaml;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

const FILES: [&'static str; 3] = ["commands.yml", "Commands.yml", "wk.yml"];
type Dictionary<T> = HashMap<String, T>;

#[derive(Deserialize, Debug)]
struct CommandsFile {
  extends: Option<Vec<PathBuf>>,
  commands: Dictionary<CommandFileDescription>,
  variables: Option<Dictionary<Primitive>>,
  environments: Option<Dictionary<Primitive>>,
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
  variables: Option<Dictionary<Primitive>>,
  environments: Option<Dictionary<Primitive>>,
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
  variables: Option<Dictionary<Primitive>>,
  description: Option<String>,
  environments: Option<Dictionary<Primitive>>,
}

#[derive(Deserialize, Debug)]
struct ConcurrentDescription {
  hidden: Option<bool>,
  depends: Option<Vec<String>>,
  commands: Vec<String>,
  variables: Option<Dictionary<Primitive>>,
  description: Option<String>,
  environments: Option<Dictionary<Primitive>>,
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

struct Importer {
  source: PathBuf,
  tasks: Dictionary<CommandImported>,
  extended_tasks: Vec<(String, ExtendedCommandDescription)>,
  extends: Option<Vec<PathBuf>>,
  commands: Option<Dictionary<CommandFileDescription>>,
  variables: Dictionary<Primitive>,
  environments: Dictionary<Primitive>,
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
      task.with_variables(p_to_s(variables));
    }
    if let Some(environments) = value.environments {
      task.with_environments(p_to_s(environments));
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
      concurrent.with_variables(p_to_s(variables));
    }
    if let Some(environments) = value.environments {
      concurrent.with_environments(p_to_s(environments));
    }

    return concurrent;
  }
}

impl From<ExtendedCommand> for CommandBuilder {
  fn from(value: ExtendedCommand) -> Self {
    let mut task = value.extend;
    task.with_cwd(value.desc.cwd);

    if let Some(args) = value.desc.args {
      task.with_args(args);
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
      task.with_variables(p_to_s(variables));
    }
    if let Some(environments) = value.desc.environments {
      task.with_environments(p_to_s(environments));
    }
    if let Some(description) = value.desc.description {
      task.with_description(description);
    }

    return task;
  }
}

impl From<CommandDescription> for ExtendedCommandDescription {
  fn from(mut value: CommandDescription) -> Self {
    let mut extend = "".to_string();
    if !is_shell_task(&value) {
      let args = split_command(value.command.as_str());
      let mut args: Vec<String> = args.iter().map(|s| (*s).to_string()).collect();

      let (_params, argv) = crate::utils::argv::parse(args.iter());
      let vars = crate::utils::argv::extract_vars(&argv);

      match value.variables.take() {
        Some(mut v) => {
          v.extend(s_to_p(vars));
          value.variables = Some(v);
        }
        None => {
          value.variables = Some(s_to_p(vars));
        }
      }

      extend = args.remove(0);
      match value.args.take() {
        Some(mut a) => {
          a.extend(args);
          value.args = Some(a);
        }
        None => {
          value.args = Some(args);
        }
      }
    }

    ExtendedCommandDescription {
      extend,
      args: value.args,
      cwd: value.cwd,
      shell: value.shell,
      hidden: value.hidden,
      depends: value.depends,
      variables: value.variables,
      environments: value.environments,
      description: value.description,
    }
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

impl From<String> for Primitive {
  fn from(value: String) -> Self {
    Primitive::S(value)
  }
}

impl std::str::FromStr for CommandDescription {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let args: Vec<&str> = s.split_whitespace().collect();
    let mut args: Vec<String> = args.iter().map(|s| (*s).into()).collect();

    if args.len() == 0 {
      return Err(Error::CommandError(
        "Cannot convert an empty string to command description".to_string(),
      ));
    }

    let command = args.remove(0);
    Ok(CommandDescription {
      command,
      args: Some(args),
      cwd: None,
      shell: None,
      hidden: None,
      depends: None,
      variables: None,
      environments: None,
      description: None,
    })
  }
}

impl Importer {
  pub fn resolve(mut self) -> Result<Context, Error> {
    let mut commands = self.commands.take().unwrap().into_iter();
    while let Some((key, value)) = commands.next() {
      match value {
        CommandFileDescription::StringCommand(command) => {
          let task_desc = command.as_str().parse::<CommandDescription>()?;
          self.add_task(key, task_desc);
        }
        CommandFileDescription::Command(task_desc) => {
          self.add_task(key, task_desc);
        }
        CommandFileDescription::Concurrent(conc_desc) => {
          let conc: Concurrent = conc_desc.into();
          self.add_concurrent(key, conc);
        }
        CommandFileDescription::ExtendedCommand(extd_desc) => {
          self.add_extend(key, extd_desc);
        }
      }
    }

    self.resolve_extends()?;
    self.to_context()
  }

  fn add_task(&mut self, name: String, cmd: CommandDescription) {
    if !is_shell_task(&cmd) {
      let extd_desc: ExtendedCommandDescription = cmd.into();
      self.add_extend(name, extd_desc);
    } else {
      self._add_task(name, cmd);
    }
  }

  fn _add_task<I>(&mut self, name: String, task_desc: I)
  where
    I: Into<CommandBuilder>,
  {
    let mut task = task_desc.into();
    let vars = task.variables.clone();
    let envs = task.environments.clone();
    task
      .with_name(name.clone())
      .with_source(&self.source)
      .with_variables(p_to_s(self.variables.clone())) // Apply file variables
      .with_variables(vars) // Override variables with task
      .with_environments(p_to_s(self.environments.clone())) // Apply file environments
      .with_environments(envs); // Override environments with task
    self.tasks.insert(name, CommandImported::Command(task));
  }

  fn add_concurrent(&mut self, name: String, mut conc: Concurrent) {
    let vars = conc.variables.clone();
    let envs = conc.environments.clone();
    conc
      .with_name(name.clone())
      .with_source(&self.source)
      .with_variables(p_to_s(self.variables.clone())) // Apply file variables
      .with_variables(vars) // Override variables with task
      .with_environments(p_to_s(self.environments.clone())) // Apply file environments
      .with_environments(envs); // Override environments with task
    self.tasks.insert(name, CommandImported::Concurrent(conc));
  }

  fn add_extend(&mut self, name: String, desc: ExtendedCommandDescription) {
    self.extended_tasks.push((name, desc));
  }

  fn resolve_extends(&mut self) -> Result<(), Error> {
    let mut pending: Vec<String> = Vec::new();
    while let Some(extd) = self.extended_tasks.pop() {
      let name = extd.0;
      let desc = extd.1;

      if let Some(cmd) = self.tasks.get(desc.extend.as_str()) {
        if let CommandImported::Command(task) = cmd {
          let extend = ExtendedCommand {
            extend: (*task).clone(),
            desc,
          };

          let mut task: CommandBuilder = extend.into();
          task.with_name(name.clone());
          self.tasks.insert(name, CommandImported::Command(task));
        } else {
          return Err(Error::ImportError(format!(
            "{} cannot extend {}.",
            name, desc.extend
          )));
        }
      } else if !pending.contains(&name) {
        pending.push(name.clone());
        self.extended_tasks.insert(0, (name, desc));
      } else {
        return Err(Error::ImportError(format!(
          "{} cannot extend {}.",
          name, desc.extend
        )));
      }
    }

    Ok(())
  }

  fn to_context(mut self) -> Result<Context, Error> {
    let mut tasks: Dictionary<CommandImported> = HashMap::new();
    for (key, value) in self.tasks {
      tasks.insert(key.to_owned(), value);
    }

    let mut context = Context { tasks, debug: 0 };

    if let Some(extends) = self.extends.take() {
      for f in extends {
        let relative_path = self.source.parent().expect("Source has no parent");
        let ff = relative_path.join(f);
        let fpath = ff.as_path();

        if fpath != self.source {
          {
            let c = load(fpath)?;
            context.extend(c);
          }
        } else {
          return Err(Error::ImportError(
            format!("Cannot extend {:?}", fpath).to_string(),
          ));
        }
      }
    }

    Ok(context)
  }
}

fn p_to_s(map: Dictionary<Primitive>) -> Dictionary<String> {
  let mut h: Dictionary<String> = HashMap::new();
  for item in map {
    h.insert(item.0, item.1.into());
  }
  return h;
}

fn s_to_p(map: Dictionary<String>) -> Dictionary<Primitive> {
  let mut h: Dictionary<Primitive> = HashMap::new();
  for item in map {
    h.insert(item.0, item.1.into());
  }
  return h;
}

fn is_shell_task(cmd: &CommandDescription) -> bool {
  let c: &str = cmd.command.as_str();
  if c.len() >= 4 && &c[0..3] == "wk:" {
    return false;
  }

  return true;
}

fn split_command(cmd: &str) -> Vec<&str> {
  let split: Vec<&str> = cmd.split_whitespace().collect();
  let mut args: Vec<&str> = Vec::new();

  let mut iterator = split.into_iter().enumerate();
  while let Some((index, arg)) = iterator.next() {
    if index == 0 {
      if arg.len() >= 4 && &arg[0..3] == "wk:" {
        let c = &arg[3..];
        args.push(c.into());
        continue;
      }
    }
    args.push(arg.into());
  }

  return args;
}

pub fn load<'a, P>(path: P) -> Result<Context, Error>
where
  P: AsRef<Path> + Copy,
{
  let content = Reader::text(path)?;
  let file: CommandsFile = serde_yaml::from_str(content.as_str())?;

  let source = PathBuf::new().join(&path);
  let importer = Importer {
    source,
    tasks: HashMap::new(),
    extended_tasks: Vec::new(),
    extends: file.extends,
    commands: Some(file.commands),
    variables: file.variables.unwrap_or(HashMap::new()),
    environments: file.environments.unwrap_or(HashMap::new()),
  };

  let c = importer.resolve()?;
  Ok(c)
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

pub fn lookup_and_load<'a, P>(dir_path: P) -> Result<Context, Error>
where
  P: AsRef<Path>,
{
  let path = lookup_dir(dir_path)?;
  load(path.as_path())
}
