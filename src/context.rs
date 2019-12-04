use std::{
  collections::HashMap,
  pin::Pin,
  task::Poll,
  task
};
use futures::{
  stream::{Stream, StreamExt},
};
use crate::{
  error::Error,
  importer::CommandImported,
  command::{Command, CommandResult, CommandBuilder},
};

pub struct Context {
  pub(crate) tasks: HashMap<String, CommandImported>,
}

impl Context {

  pub fn find_builder(&self, name: &str) -> Option<&CommandBuilder> {
    if let Some(imported) = self.tasks.get(name) {
      match imported {
        CommandImported::Command(builder) => {
          return Some(builder);
        },
        _ => {}
      }
    }

    None
  }

  pub fn find_command(&self, name: &str) -> Option<Command> {
    if let Some(builder) = self.find_builder(name) {
      return Some(builder.to_command());
    }

    None
  }

  pub async fn run(&self, name: &str) -> Result<Vec<CommandResult>, Error> {
    if let Some(command) = self.find_command(name) {

      let mut stack: Vec<Command> = Vec::new();

      // Add dependencies
      if !command.dependencies.is_empty() {
        for depname in &command.dependencies {
          match self.find_command(depname.as_str()) {
            Some(dep) => {
              stack.push(dep);
            },
            None => {}
          }
        }
      }

      // Add current command
      stack.push(command);

      // Run commands
      let mut results: Vec<CommandResult> = Vec::new();
      for c in stack.into_iter() {
        results.push(c.execute().await);
      }

      return Ok(results);
    }

    let err = format!("Command \"{}\" not found", name);
    Err(Error::CommandError(err))
  }

}