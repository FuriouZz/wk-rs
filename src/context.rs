use crate::{
  command::{Command, CommandBuilder, CommandResult},
  error::Error,
  importer::CommandImported,
};
use std::collections::HashMap;

pub struct Context {
  pub(crate) tasks: HashMap<String, CommandImported>,
}

impl Context {
  pub fn find_builder<S>(&self, name: S) -> Option<&CommandBuilder>
  where
    S: AsRef<str>,
  {
    if let Some(imported) = self.tasks.get(name.as_ref()) {
      match imported {
        CommandImported::Command(builder) => {
          return Some(builder);
        }
        _ => {}
      }
    }

    None
  }

  pub fn find_command<S>(&self, name: S) -> Option<Command>
  where
    S: AsRef<str>,
  {
    if let Some(builder) = self.find_builder(name) {
      return Some(builder.to_command());
    }

    None
  }

  pub async fn run<S>(&self, name: S) -> Result<Vec<CommandResult>, Error>
  where
    S: AsRef<str>,
  {
    if let Some(command) = self.find_command(name.as_ref()) {
      let mut stack: Vec<Command> = Vec::new();

      // Add dependencies
      if !command.dependencies.is_empty() {
        for depname in &command.dependencies {
          match self.find_command(depname.as_str()) {
            Some(dep) => {
              stack.push(dep);
            }
            None => {}
          }
        }
      }

      // Add current command
      stack.push(command);

      // Run commands
      let mut results: Vec<CommandResult> = Vec::new();
      for c in stack.into_iter() {
        results.push(c.execute(&self).await);
      }

      return Ok(results);
    }

    let err = format!("Command \"{}\" not found", name.as_ref().to_string());
    Err(Error::CommandError(err))
  }
}
