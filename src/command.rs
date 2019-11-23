#[derive(Debug, Clone)]
pub enum CommandKind {
  WK,
  Shell,
}

#[derive(Debug, Clone)]
pub struct Command {
  cwd: Option<std::path::PathBuf>,
  args: Vec<String>,
  name: String,
  kind: CommandKind,
  hidden: bool,
  source: std::path::PathBuf,
  command: String,
  variables: std::collections::HashMap<String, String>,
  description: Option<String>,
  dependencies: Vec<String>,
}

impl Command {

  pub fn new() -> Self {
    Self {
      cwd: None,
      args: Vec::new(),
      name: String::from("command"),
      kind: CommandKind::Shell,
      hidden: false,
      source: std::path::PathBuf::new(),
      command: String::from(""),
      variables: std::collections::HashMap::new(),
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

  #[allow(dead_code)]
  pub fn with_command<S>(&mut self, command: S) -> &mut Self
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
          self.kind = CommandKind::WK;
          self.command = reg.replace(param, "").into();
        } else {
          self.kind = CommandKind::Shell;
          self.command = param.into();
        }
      } else {
        self.args.push(param.into());
      }
    }

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
    S: Into<std::path::PathBuf>,
  {
    self.cwd = cwd.map(|s| s.into());
    self
  }

  pub fn with_source<S>(&mut self, source: S) -> &mut Self
  where
    S: Into<std::path::PathBuf>,
  {
    self.source = source.into();
    self
  }

  pub fn with_hidden(&mut self, hidden: bool) -> &mut Self {
    self.hidden = hidden;
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
    I: IntoIterator<Item=S>,
    S: Into<String>,
  {
    for dependency in dependencies {
      self.with_dependency(dependency);
    }
    self
  }

  #[allow(dead_code)]
  pub fn with_arg<S>(&mut self, arg: S) -> &mut Self
  where
    S: Into<String>,
  {
    self.args.push(arg.into());
    self
  }

  pub fn with_args<I, S>(&mut self, args: I) -> &mut Self
  where
    I: IntoIterator<Item=S>,
    S: Into<String>,
  {
    for arg in args {
      self.with_arg(arg);
    }
    self
  }

  pub fn with_variables(&mut self, variables: std::collections::HashMap<String, String>) -> &mut Self {
    self.variables.extend(variables);
    self
  }

}

impl std::str::FromStr for Command {
  type Err = std::str::Utf8Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut command = Command::new();
    command.with_command(s);
    Ok(command)
  }
}