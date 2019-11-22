#[derive(Debug, Clone)]
pub enum TaskKind {
  WK,
  Shell,
}

#[derive(Debug, Clone)]
pub struct Task {
  cwd: Option<std::path::PathBuf>,
  args: Vec<String>,
  name: String,
  kind: TaskKind,
  hidden: bool,
  source: std::path::PathBuf,
  command: String,
  variables: std::collections::HashMap<String, String>,
  description: Option<String>,
  dependencies: Vec<String>,
}

impl Task {

  pub fn new() -> Self {
    Self {
      cwd: None,
      args: Vec::new(),
      name: String::from("task"),
      kind: TaskKind::Shell,
      hidden: false,
      source: std::path::PathBuf::new(),
      command: String::from(""),
      variables: std::collections::HashMap::new(),
      description: None,
      dependencies: Vec::new(),
    }
  }

  pub fn with_name<S>(mut self, name: S) -> Self
  where
    S: Into<String>,
  {
    self.name = name.into();
    self
  }

  #[allow(dead_code)]
  pub fn with_command<S>(mut self, command: S) -> Self
  where
    S: Into<String>,
  {
    let cmd = command.into();
    let parameters: Vec<&str> = cmd.split_whitespace().collect();

    let mut iterator = parameters.into_iter().enumerate();
    while let Some((index, param)) = iterator.next() {
      if index == 0 {
        let reg = regex::Regex::new("^wk:").unwrap();
        println!("TODO: Regex Static");
        if reg.is_match(param) {
          self.kind = TaskKind::WK;
          self.command = reg.replace(param, "").into();
        } else {
          self.kind = TaskKind::Shell;
          self.command = param.into();
        }
      } else {
        self.args.push(param.into());
      }
    }

    self
  }

  pub fn with_description<S>(mut self, description: S) -> Self
  where
    S: Into<String>,
  {
    self.description = Some(description.into());
    self
  }

  pub fn with_cwd<S>(mut self, cwd: Option<S>) -> Self
  where
    S: Into<std::path::PathBuf>,
  {
    self.cwd = cwd.map(|s| s.into());
    self
  }

  pub fn with_source<S>(mut self, source: S) -> Self
  where
    S: Into<std::path::PathBuf>,
  {
    self.source = source.into();
    self
  }

  pub fn with_hidden(mut self, hidden: bool) -> Self {
    self.hidden = hidden;
    self
  }

  pub fn with_dependency<S>(mut self, dependency: S) -> Self
  where
    S: Into<String>,
  {
    self.dependencies.push(dependency.into());
    self
  }

  pub fn with_dependencies<I, S>(mut self, dependencies: I) -> Self
  where
    I: IntoIterator<Item=S>,
    S: Into<String>,
  {
    for dependency in dependencies {
      self = self.with_dependency(dependency);
    }
    self
  }

  #[allow(dead_code)]
  pub fn with_arg<S>(mut self, arg: S) -> Self
  where
    S: Into<String>,
  {
    self.args.push(arg.into());
    self
  }

  pub fn with_args<I, S>(mut self, args: I) -> Self
  where
    I: IntoIterator<Item=S>,
    S: Into<String>,
  {
    for arg in args {
      self = self.with_arg(arg);
    }
    self
  }

  pub fn with_variables(mut self, variables: std::collections::HashMap<String, String>) -> Self {
    self.variables.extend(variables);
    self
  }

}

impl std::str::FromStr for Task {
  type Err = std::str::Utf8Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Task::new().with_command(s))
  }
}