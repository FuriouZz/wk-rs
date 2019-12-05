#[derive(Debug)]
pub struct Concurrent {
  name: String,
  source: std::path::PathBuf,
  hidden: bool,
  commands: Vec<String>,
  pub(crate) variables: std::collections::HashMap<String, String>,
  description: Option<String>,
  dependencies: Vec<String>,
}

impl Concurrent {
  pub fn new() -> Self {
    Self {
      name: String::from("task"),
      source: std::path::PathBuf::new(),
      hidden: false,
      commands: Vec::new(),
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

  pub fn with_description<S>(&mut self, description: S) -> &mut Self
  where
    S: Into<String>,
  {
    self.description = Some(description.into());
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
    I: IntoIterator<Item = S>,
    S: Into<String>,
  {
    for dependency in dependencies {
      self.with_dependency(dependency);
    }
    self
  }

  pub fn with_command<S>(&mut self, command: S) -> &mut Self
  where
    S: Into<String>,
  {
    self.commands.push(command.into());
    self
  }

  pub fn with_commands<I, S>(&mut self, commands: I) -> &mut Self
  where
    I: IntoIterator<Item = S>,
    S: Into<String>,
  {
    for command in commands {
      self.with_command(command);
    }
    self
  }

  pub fn with_variables(
    &mut self,
    variables: std::collections::HashMap<String, String>,
  ) -> &mut Self {
    self.variables.extend(variables);
    self
  }
}
