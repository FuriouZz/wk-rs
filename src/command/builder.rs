use super::command::Command;
use crate::error::Error;
use std::{
  collections::HashMap,
  env,
  path::PathBuf,
  str::FromStr,
};

#[derive(Debug, Clone)]
pub struct CommandBuilder {
  cwd: Option<PathBuf>,
  args: Vec<String>,
  name: String,
  shell: Option<PathBuf>,
  hidden: bool,
  source: PathBuf,
  pub(crate) variables: HashMap<String, String>,
  pub(crate) environments: HashMap<String, String>,
  description: Option<String>,
  dependencies: Vec<String>,
}

impl CommandBuilder {
  pub fn new() -> Self {
    Self {
      cwd: None,
      args: Vec::new(),
      name: String::from("command"),
      shell: None,
      hidden: false,
      source: PathBuf::new(),
      variables: HashMap::new(),
      environments: HashMap::new(),
      description: None,
      dependencies: Vec::new(),
    }
  }

  pub fn with_name<S>(&mut self, name: S) -> &mut Self
  where
    S: Into<String>,
  {
    self.name = name.into();
    self
  }

  pub fn with_command<S>(&mut self, command: S) -> &mut Self
  where
    S: Into<String>,
  {
    let cmd = command.into();
    let parameters = cmd
      .split_whitespace()
      .collect::<Vec<&str>>()
      .iter()
      .map(|s| (*s).into())
      .collect::<Vec<String>>();

    self.args.clear();
    self.args.extend(parameters);

    self
  }

  pub fn with_description<S>(&mut self, description: S) -> &mut Self
  where
    S: Into<String>,
  {
    self.description = Some(description.into());
    self
  }

  pub fn with_cwd<S>(&mut self, cwd: Option<S>) -> &mut Self
  where
    S: Into<PathBuf>,
  {
    self.cwd = cwd.map(|s| s.into());
    self
  }

  pub fn with_source<S>(&mut self, source: S) -> &mut Self
  where
    S: Into<PathBuf>,
  {
    self.source = source.into();
    self
  }

  pub fn with_hidden(&mut self, hidden: bool) -> &mut Self {
    self.hidden = hidden;
    self
  }

  pub fn with_shell<S>(&mut self, shell: S) -> &mut Self
  where
    S: Into<PathBuf>,
  {
    self.shell = Some(shell.into());
    self
  }

  pub fn with_dependency<S>(&mut self, dependency: S) -> &mut Self
  where
    S: Into<String>,
  {
    self.dependencies.push(dependency.into());
    self
  }

  pub fn with_dependencies<I, S>(&mut self, dependencies: I) -> &mut Self
  where
    I: IntoIterator<Item = S>,
    S: Into<String>,
  {
    for dependency in dependencies {
      self.with_dependency(dependency);
    }
    self
  }

  pub fn with_arg<S>(&mut self, arg: S) -> &mut Self
  where
    S: Into<String>,
  {
    self.args.push(arg.into());
    self
  }

  pub fn with_args<I, S>(&mut self, args: I) -> &mut Self
  where
    I: IntoIterator<Item = S>,
    S: Into<String>,
  {
    for arg in args {
      self.with_arg(arg);
    }
    self
  }

  pub fn override_args<I, S>(&mut self, args: I) -> &mut Self
  where
    I: IntoIterator<Item = S>,
    S: Into<String>,
  {
    self.args.clear();
    for arg in args {
      self.with_arg(arg);
    }
    self
  }

  pub fn with_variables(&mut self, variables: HashMap<String, String>) -> &mut Self {
    self.variables.extend(variables);
    self
  }

  pub fn with_environments(&mut self, environments: HashMap<String, String>) -> &mut Self {
    self.environments.extend(environments);
    self
  }

  pub fn to_command(&self, variables: Option<&HashMap<String, String>>) -> Command {
    // Set variables
    let mut vars = HashMap::new();
    vars.extend(&self.variables);
    if let Some(v) = variables {
      vars.extend(v);
    }

    // Set arguments
    let mut args: Vec<String> = self
      .args
      .iter()
      .map(|arg: &String| {
        let mut arg_res = arg.to_string();

        for (key, value) in vars.iter() {
          let r_key = format!("${{{}}}", key);
          arg_res = arg_res.as_str().replace(r_key.as_str(), value);
        }

        arg_res
      })
      .collect();

    // Set CWD
    let mut cwd: Option<PathBuf> = None;
    if let Some(ccwd) = &self.cwd {
      cwd = Some(PathBuf::new().join(ccwd));
    } else if let Ok(ccwd) = env::current_dir() {
      cwd = Some(ccwd);
    } else {
      if self.source.is_file() {
        if let Some(dir) = self.source.parent() {
          cwd = Some(PathBuf::new().join(dir));
        }
      }
    }

    // Set Shell
    let shell = {
      if let Some(shell) = &self.shell {
        args.insert(0, "-c".into());
        PathBuf::new().join(shell)
      } else {
        if cfg!(windows) {
          args.insert(0, "/c".into());
          PathBuf::new().join("cmd.exe")
        } else {
          args.insert(0, "-c".into());
          PathBuf::new().join("bash")
        }
      }
    };

    // Set dependencies
    let dependencies = self.dependencies.clone();

    // Set environments
    let environments = self.environments.clone();

    Command {
      name: self.name.clone(),
      cwd,
      args,
      shell,
      environments,
      dependencies,
    }
  }
}

impl FromStr for CommandBuilder {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s.is_empty() {
      return Err(Error::Command(
        "Cannot convert an empty string to command".to_string(),
      ));
    }

    let mut command = CommandBuilder::new();
    command.with_command(s);
    Ok(command)
  }
}
