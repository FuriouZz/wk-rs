use crate::{
  command::{Command, CommandBuilder, CommandResult},
  error::Error,
  importer::CommandImported,
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Context {
  pub(crate) tasks: HashMap<String, CommandImported>,
  pub(crate) debug: i32,
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

  pub fn create_command<S>(
    &self,
    name: S,
    variables: Option<&HashMap<String, String>>,
  ) -> Option<Command>
  where
    S: AsRef<str>,
  {
    if let Some(builder) = self.find_builder(name) {
      return Some(builder.to_command(variables));
    }

    None
  }

  pub fn create_stack<S>(
    &self,
    name: S,
    order: &mut Vec<String>,
    tasks: &mut HashMap<String, Command>,
    variables: Option<&HashMap<String, String>>,
  ) where
    S: AsRef<str>,
  {
    let name_ref = name.as_ref();

    if let Some(command) = self.create_command(name_ref, variables) {
      // Add dependencies
      if !command.dependencies.is_empty() {
        for depname in &command.dependencies {
          match self.find_builder(&depname) {
            Some(_dep) => {
              if depname != name_ref && !tasks.contains_key(depname) {
                self.create_stack(depname, order, tasks, variables);
              }
            }
            None => {}
          }
        }
      }

      // Add task if it does not exist
      let name_owned = name_ref.to_owned();
      if !tasks.contains_key(&name_owned) {
        order.push(name_owned.clone());
        tasks.insert(name_owned, command);
      }
    }
  }

  pub async fn run<S>(
    &self,
    name: S,
    variables: Option<&HashMap<String, String>>,
  ) -> Result<Vec<CommandResult>, Error>
  where
    S: AsRef<str>,
  {
    let name_ref = name.as_ref();

    if let None = self.find_builder(name_ref) {
      let err = format!("Command \"{}\" not found", name_ref);
      return Err(Error::Command(err));
    }

    let mut order: Vec<String> = Vec::new();
    let mut commands: HashMap<String, Command> = HashMap::new();
    self.create_stack(name_ref, &mut order, &mut commands, variables);

    // Run commands
    let mut results: Vec<CommandResult> = Vec::new();
    for taskname in order.iter() {
      if let Some(c) = commands.remove(taskname) {
        match self.debug {
          2 => {
            c.display();
          }
          1 => {
            c.debug();
            results.push(c.execute().await)
          }
          _ => results.push(c.execute().await),
        }
      }
    }

    Ok(results)
  }

  pub fn extend(&mut self, context: Context) {
    for task in context.tasks {
      self.tasks.insert(task.0, task.1);
    }
  }
}
