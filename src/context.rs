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

  pub fn create_stack<'a, S>(
    &'a self,
    name: S,
    tasks: &mut Vec<Command<'a>>,
    variables: Option<&HashMap<String, String>>,
  )
  where
    S: AsRef<str>,
  {
    let name_ref = name.as_ref();

    if let Some(command) = self.create_command(name_ref, variables) {
      // Add dependencies
      if !command.dependencies.is_empty() {
        for depname in command.dependencies {
          match self.find_builder(&depname) {
            Some(_dep) => {

              let found = tasks
              .iter()
              .filter(|item| &item.name == depname)
              .take(1)
              .next();

              if depname != name_ref && found.is_none() {
                self.create_stack(depname, tasks, variables);
              }
            }
            None => {}
          }
        }
      }

      let found = tasks
      .iter()
      .filter(|item| item.name == name_ref)
      .take(1)
      .next();

      if found.is_none() {
        tasks.push(command);
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

    let mut commands: Vec<Command> = Vec::new();
    self.create_stack(name_ref, &mut commands, variables);

    // Run commands
    let mut results: Vec<CommandResult> = Vec::new();
    for c in commands.into_iter() {
      // c.display();
      // results.push(c.execute().await);
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

    Ok(results)
  }

  pub fn extend(&mut self, context: Context) {
    for task in context.tasks {
      self.tasks.insert(task.0, task.1);
    }
  }
}
