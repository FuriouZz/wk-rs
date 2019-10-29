use std::path::PathBuf;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum TaskVisibility {
  Visible,
  Hidden,
}

#[derive(Debug, Clone)]
pub struct Task {
  command: String,
  cwd: PathBuf,
  name: String,
  source: PathBuf,
  visible: TaskVisibility,
  bin_path: PathBuf,
  variables: HashMap<String, String>,
  parameters: Vec<String>,
  description: Option<String>,
  dependencies: Vec<String>,
}

impl Task {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn name<S>(mut self, name: S) -> Self
  where
    S: Into<String>,
  {
    self.name = name.into();
    self
  }

  pub fn description<S>(mut self, description: S) -> Self
  where
    S: Into<String>,
  {
    self.description = Some(description.into());
    self
  }

  pub fn command<S>(mut self, command: S) -> Self
  where
    S: Into<String>,
  {
    let cmd = command.into();
    let parameters: Vec<&str> = cmd.split_whitespace().collect();

    let mut iterator = parameters.into_iter().enumerate();
    while let Some((index, param)) = iterator.next() {
      if index == 0 {
        self.command = param.into();
      } else {
        self.parameters.push(param.into());
      }
    }

    self
  }

  pub fn cwd<S>(mut self, cwd: S) -> Self
  where
    S: Into<PathBuf>,
  {
    self.cwd = cwd.into();
    self
  }

  pub fn source<S>(mut self, source: S) -> Self
  where
    S: Into<PathBuf>,
  {
    self.source = source.into();
    self
  }

  pub fn bin_path<S>(mut self, bin_path: S) -> Self
  where
    S: Into<PathBuf>,
  {
    self.bin_path = bin_path.into();
    self
  }

  pub fn visible(mut self, visible: TaskVisibility) -> Self {
    self.visible = visible;
    self
  }

  pub fn depend<S>(mut self, dependency: S) -> Self
  where
    S: Into<String>,
  {
    self.dependencies.push(dependency.into());
    self
  }

  pub fn param<S>(mut self, parameter: S) -> Self
  where
    S: Into<String>,
  {
    self.parameters.push(parameter.into());
    self
  }

  pub fn variables(mut self, variables: HashMap<String, String>) -> Self {
    self.variables.extend(variables);
    self
  }
}

impl Default for Task {
  fn default() -> Self {
    Self {
      command: String::new(),
      cwd: PathBuf::new(),
      name: String::from("task"),
      source: PathBuf::new(),
      visible: TaskVisibility::Visible,
      bin_path: PathBuf::new(),
      variables: HashMap::new(),
      parameters: Vec::new(),
      description: None,
      dependencies: Vec::new(),
    }
  }
}

impl FromStr for Task {
  type Err = std::str::Utf8Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let task = Task::new().command(s);
    Ok(task)
  }
}
